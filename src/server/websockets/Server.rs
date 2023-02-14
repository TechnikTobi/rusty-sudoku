use actix::{prelude::*};
use actix_broker::{BrokerSubscribe};

use std::collections::HashMap;

use crate::messages::outgoing::PlayerRegistrationResponse::PlayerRegistrationResponse;
use crate::messages::base::NetworkPlayerIdentifier::NetworkPlayerIdentifier;

use super::Messages::*;


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
			if let Err(error) = client.try_send(message.to_owned())
			{
				println!("Oh no! Something went wrong with sending data to all players! {:?}", error);
			}
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