use actix::fut;
use actix::prelude::*;
use actix_web::web;
use actix_web_actors::ws;

use std::sync::Mutex;

use crate::board::field::Field;
use crate::board::difficulty::Difficulty;
use crate::game::GameID::GameID;
use crate::game::player::PlayerID::PlayerID;

use crate::messages::base::NetworkGameIdentifier::NetworkGameIdentifier;
use crate::messages::base::NetworkPlayerIdentifier::NetworkPlayerIdentifier;
use crate::messages::incoming::PlayerRegistrationRequest::PlayerRegistrationRequest;
use crate::messages::incoming::GameCreationRequest::GameCreationRequest;
use crate::messages::incoming::GameJoinLeaveRequest::GameJoinLeaveRequest;
use crate::messages::incoming::GameReadyUnreadyRequest::GameReadyUnreadyRequest;
use crate::messages::incoming::GameSetFieldRequest::GameSetFieldRequest;

use crate::server::Server::SudokuServer;

use super::Messages::*;
use super::Server::WebSocketServer;

#[derive(Default)]
pub struct
WebsocketSession
{
	server: Option<web::Data<Mutex<SudokuServer>>>
}

impl
Actor 
for 
WebsocketSession
{

	type Context = ws::WebsocketContext<Self>;

	fn started
	(
		&mut self, 
		_ctx: &mut Self::Context
	) 
	{
		println!("Started a WebsocketSession!");
	}

	fn stopped
	(
		&mut self, 
		_ctx: &mut Self::Context
	) 
	{
		println!("Closed a WebsocketSession!");
	}
}

impl
WebsocketSession
{

	pub fn
	new
	(
		server: Option<web::Data<Mutex<SudokuServer>>>
	)
	-> Self
	{
		WebsocketSession
		{
			server: server
		}
	}

	fn
	issue_internal_list_update_message
	(
		&mut self,
		context: &mut <WebsocketSession as Actor>::Context,
	)
	{
		let message = InternalGameListUpdateMessage(
			self.server
				.as_ref()
				.unwrap()
				.lock()
				.unwrap()
				.generate_games_list_response()
		);

		WebSocketServer::from_registry().send(message)
			.into_actor(self)
			.then(|_, _, _| { fut::ready(()) })
			.wait(context);
	}

	fn
	issue_internal_game_join_leave_message
	(
		&mut self,
		context: &mut <WebsocketSession as Actor>::Context,
		game_id: &NetworkGameIdentifier,
		player_id: &NetworkPlayerIdentifier,
	)
	{
		let message = InternalGameJoinLeaveMessage(
			game_id.to_owned(), 
			player_id.to_owned()
		);

		WebSocketServer::from_registry().send(message)
			.into_actor(self)
			.then(|_, _, _| { fut::ready(()) })
			.wait(context);	
	}

	fn
	issue_internal_game_update_message
	(
		&mut self,
		context: &mut <WebsocketSession as Actor>::Context,
		game_id: &NetworkGameIdentifier,
	)
	{
		let immut_server = self.server
			.as_ref()
			.unwrap()
			.lock()
			.unwrap();

		let (game_state, player_list) = immut_server
			.get_game_controller_manager()
			.get_game(&GameID::from_network(game_id))
			.unwrap()
			.to_network(immut_server.get_player_manager(), "".to_string());

		let message = InternalGameUpdateMessage(game_state, player_list);

		WebSocketServer::from_registry().send(message)
			.into_actor(self)
			.then(|_, _, _| { fut::ready(()) })
			.wait(context);	
	}
}

impl
Handler<JsonMessage>
for
WebsocketSession
{
	type Result = ();

	fn
	handle
	(
		&mut self,
		message: JsonMessage,
		context: &mut Self::Context
	)
	{
		context.text(message.0);
	}
}

impl 
StreamHandler<Result<ws::Message, ws::ProtocolError>> 
for 
WebsocketSession
{
	fn handle
	(
		&mut self, 
		incoming_message: Result<ws::Message, ws::ProtocolError>, 
		context: &mut Self::Context
	) 
	{
		let message = match incoming_message 
		{
			Err(_) => {
				context.stop();
				return;
			}
			Ok(message) => message,
		};

		println!("WEBSOCKET MESSAGE {:?}", message);

		match message 
		{
			ws::Message::Text(message_text) => {

				let text = message_text.trim();				

				// Try different parsings of the data inside the 
				if let Ok(request) = serde_json::from_str::<PlayerRegistrationRequest>(text)
				{

					// Don't allow empty player names
					if request.get_player_name().trim().is_empty()
					{
						return;
					}

					// Create a new player in the SudokuServer
					let new_player_id = self.server
						.as_ref()
						.unwrap()
						.lock()
						.unwrap()
						.get_mut_player_manager()
						.add_player(request.get_player_name().to_owned());

					// Create a new internal registration message to send to the
					// Websocket Server instance
					let registration = InternalPlayerRegistrationMessage(
						new_player_id.to_network(),
						context.address().recipient(),
						self.server.as_ref().unwrap().lock().unwrap().generate_games_list_response()
					);

					// Send the internal registration to the Websocket Server
					WebSocketServer::from_registry().send(registration)
						.into_actor(self)
						.then(|_, _, _| { fut::ready(()) })
						.wait(context);
				}
				else if let Ok(request) = serde_json::from_str::<GameSetFieldRequest>(text)
				{
					// First, get the player
					let player = self.server
						.as_ref()
						.unwrap()
						.lock()
						.unwrap()
						.get_player_manager()
						.get_player(&PlayerID::from_network(request.get_player_id()))
						.unwrap()
						.clone();

					// Then use the player & the field to try to set its value
					self.server
						.as_ref()
						.unwrap()
						.lock()
						.unwrap()
						.get_mut_game_controller_manager()
						.get_mut_game(&GameID::from_network(request.get_game_id()))
						.unwrap()
						.set_field(
							Field::from_network(request.get_field()),
							&player
						);

					// Finally, notify all participating players that something changed
					self.issue_internal_game_update_message(context, request.get_game_id());
				}
				else if let Ok(request) = serde_json::from_str::<GameCreationRequest>(text)
				{

					// Don't allow empty game names
					// Don't allow games with invalid difficulty
					let given_difficulty = request.get_difficulty();
					let bound_difficulty = Difficulty::bound_difficulty(*given_difficulty);

					if 
						(request.get_game_name().trim().is_empty()) ||
						(given_difficulty != &bound_difficulty)
					{
						return;
					}

					// Create a new game in the SudokuServer
					let new_game_id = self.server
						.as_ref()
						.unwrap()
						.lock()
						.unwrap()
						.get_mut_game_controller_manager()
						.create_game(
							PlayerID::from_network(request.get_player_id()), 
							request.get_game_name().clone(), 
							request.get_difficulty().clone()
					);

					// Automatically join the Master
					self.server
						.as_ref()
						.unwrap()
						.lock()
						.unwrap()
						.get_mut_game_controller_manager()
						.get_mut_game(&new_game_id)
						.unwrap()
						.toggle_player(PlayerID::from_network(request.get_player_id()));

					self.issue_internal_list_update_message(context);
					self.issue_internal_game_join_leave_message(context, &new_game_id.to_network(), &request.get_player_id());
					self.issue_internal_game_update_message(context, &new_game_id.to_network());
				}
				else if let Ok(request) = serde_json::from_str::<GameJoinLeaveRequest>(text)
				{
					// TODO:
					// - De-Join Client from any other game

					// Add or remove the client to the game
					let return_game_id = self.server
						.as_ref()
						.unwrap()
						.lock()
						.unwrap()
						.get_mut_game_controller_manager()
						.get_mut_game(&GameID::from_network(request.get_game_id()))
						.unwrap()
						.toggle_player(PlayerID::from_network(request.get_player_id()));

					self.issue_internal_list_update_message(context);
					self.issue_internal_game_join_leave_message(context, &return_game_id.to_network(), &request.get_player_id());
					self.issue_internal_game_update_message(context, request.get_game_id());
				}
				else if let Ok(request) = serde_json::from_str::<GameReadyUnreadyRequest>(text)
				{
					self.server
						.as_ref()
						.unwrap()
						.lock()
						.unwrap()
						.get_mut_game_controller_manager()
						.get_mut_game(&GameID::from_network(request.get_game_id()))
						.unwrap()
						.ready_player(PlayerID::from_network(request.get_player_id()));

					self.issue_internal_game_update_message(context, request.get_game_id());
				}
				else
				{
					println!("Some other kind of message...");
					println!("{:?}", text);
				}
			}
			ws::Message::Close(reason) => {

				// TODO: Cleanup!

				context.close(reason);
				context.stop();
			}
			_ => {}
		}
	}
}