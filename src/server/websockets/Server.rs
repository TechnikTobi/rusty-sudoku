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

	pub fn
	send_game_message
	(
		&self,
		message: JsonMessage,
		players: Vec<NetworkPlayerIdentifier>
	)
	{
		for player in players
		{
			if let Err(error) = self.clients.get(&player).unwrap().try_send(message.to_owned())
			{
				println!("Oh no! Something went wrong with sending the game state! {:?}", error);
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
		let InternalPlayerRegistrationMessage(network_player_id, recipient, games_list) = msg;

		// Add the recipient to the list of clients
		self.clients.insert(network_player_id.clone(), recipient.clone());
		
		// Prepare messages containing the new player's ID and a list of current games
		let player_id_message = JsonMessage(
			serde_json::to_string(&PlayerRegistrationResponse::new(network_player_id, "".to_string())).unwrap(), 
			None
		);
		let games_list_message = JsonMessage(
			serde_json::to_string(&games_list).unwrap(), 
			None
		);

		// Send the messages
		// If that results in an error, currently handle this by printing something to CLI
		if let Err(error) = recipient.try_send(player_id_message)
		{
			println!("Oh no! Something went wrong with sending the new player their ID! {:?}", error);
		}

		if let Err(error) = recipient.try_send(games_list_message)
		{
			println!("Oh no! Something went wrong with sending the new player the list of games! {:?}", error);
		}

		// Still not sure why we return... this... thing
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

impl
Handler<InternalGameJoinLeaveMessage>
for
WebSocketServer
{
	type Result = MessageResult<InternalGameJoinLeaveMessage>;

	fn handle(
		&mut self, 
		msg: InternalGameJoinLeaveMessage, 
		_ctx: &mut Self::Context
	) 
	-> Self::Result 
	{
		// Deconstruct the internal message
		let InternalGameJoinLeaveMessage(games_list, game_state, player_list) = msg;

		// 1. Send to EVERYONE the new list of games (due to change of numbers in table)
		self.send_to_all(JsonMessage(
			serde_json::to_string(&games_list).unwrap(), 
			None
		));

		// 2. Send updated game state to relevant clients
		self.send_game_message(
			JsonMessage(serde_json::to_string(&game_state).unwrap(), None),
			player_list
		);

		// See above
		MessageResult(())
	}
}