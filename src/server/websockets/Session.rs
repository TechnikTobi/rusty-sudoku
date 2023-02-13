use actix::{fut, prelude::*};
use actix_broker::{BrokerIssue, BrokerSubscribe};
use actix_web::web;
use actix_web_actors::ws;

use std::{sync::Mutex, collections::HashMap};

use crate::game::player::PlayerID::PlayerID;
use crate::messages::outgoing::PlayerRegistrationResponse::PlayerRegistrationResponse;
use crate::server::Server::SudokuServer;
use crate::messages::base::NetworkPlayerIdentifier::NetworkPlayerIdentifier;
use crate::messages::base::NetworkGameIdentifier::NetworkGameIdentifier;
use crate::messages::incoming::GameCreationRequest::GameCreationRequest;
use crate::messages::incoming::PlayerRegistrationRequest::PlayerRegistrationRequest;

use super::Messages::*;

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
						context.address().recipient()
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




#[derive(Default)]
pub struct
WebSocketServer
{
	// clients: HashMap<NetworkPlayerIdentifier, Recipient<WebsocketGamesList>>,
	// games_clients: HashMap<NetworkGameIdentifier, Vec<Recipient<GameUpdateMessage>>>,
	clients: HashMap<NetworkPlayerIdentifier, Recipient<JsonMessage>>,
}

impl
WebSocketServer
{
	pub fn 
	send_to_all
	(
		&self,
		message: JsonMessage
	)
	{
		for (_, client) in &self.clients
		{
			client.try_send(message.to_owned());
		}
	}
}

impl SystemService for WebSocketServer {}
impl Supervised for WebSocketServer {}

impl 
Actor 
for 
WebSocketServer 
{
	type Context = Context<Self>;

	fn 
	started
	(
		&mut self, 
		context: &mut Self::Context
	) 
	{
		self.subscribe_system_async::<InternalPlayerRegistrationMessage>(context);
		self.subscribe_system_async::<InternalGameCreationMessage>(context);
	}
}

impl 
Handler<InternalPlayerRegistrationMessage> 
for 
WebSocketServer 
{
	type Result = MessageResult<InternalPlayerRegistrationMessage>;

	fn handle(
		&mut self, 
		msg: InternalPlayerRegistrationMessage, 
		_ctx: &mut Self::Context
	) 
	-> Self::Result 
	{
		// Deconstruct the internal message
		let InternalPlayerRegistrationMessage(network_player_id, recipient) = msg;

		// Add the recipient to the list of clients
		self.clients.insert(network_player_id.clone(), recipient.clone());
		
		// Send them back their player ID
		let send_result = recipient.try_send(JsonMessage(
			serde_json::to_string(&PlayerRegistrationResponse::new(network_player_id, "".to_string())).unwrap(), 
			None
		));

		// If that results in an error, currently handle this by printing something to CLI
		if let Err(error) = send_result
		{
			println!("Oh no! Something went wrong with sending the new player their ID! {:?}", error);
		}

		// Still not sure why we return... this
		MessageResult(())
	}
}

impl 
Handler<InternalGameCreationMessage> 
for 
WebSocketServer 
{
	type Result = MessageResult<InternalGameCreationMessage>;

	fn handle(
		&mut self, 
		msg: InternalGameCreationMessage, 
		_ctx: &mut Self::Context
	) 
	-> Self::Result 
	{
		// Deconstruct the internal message
		let InternalGameCreationMessage(games_list) = msg;

		// Send to EVERYONE
		self.send_to_all(JsonMessage(
			serde_json::to_string(&games_list).unwrap(), 
			None
		));

		// See above
		MessageResult(())
	}
}