use actix::{fut, prelude::*};
use actix_web::web;
use actix_web_actors::ws;

use std::sync::Mutex;

use crate::game::player::PlayerID::PlayerID;
use crate::server::Server::SudokuServer;
use crate::messages::incoming::GameCreationRequest::GameCreationRequest;
use crate::messages::incoming::PlayerRegistrationRequest::PlayerRegistrationRequest;

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
				else if let Ok(request) = serde_json::from_str::<GameCreationRequest>(text)
				{
					// Create a new game in the SudokuServer
					let _new_game_id = self.server
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

					// Send the internal registration to the Websocket Server
					let games_list_message = InternalGameCreationMessage(
						self.server.as_ref().unwrap().lock().unwrap().generate_games_list_response()
					);
					WebSocketServer::from_registry().send(games_list_message)
						.into_actor(self)
						.then(|_, _, _| { fut::ready(()) })
						.wait(context);
				}
				else
				{
					println!("Some other kind of message...");
					println!("{:?}", text);
				}
			}
			ws::Message::Close(reason) => {
				context.close(reason);
				context.stop();
			}
			_ => {}
		}
	}
}