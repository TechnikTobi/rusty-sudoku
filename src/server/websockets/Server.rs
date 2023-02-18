use actix::prelude::*;
use actix_broker::BrokerSubscribe;

use std::collections::HashMap;

use crate::messages::outgoing::PlayerRegistrationResponse::PlayerRegistrationResponse;
use crate::messages::base::NetworkPlayerIdentifier::NetworkPlayerIdentifier;

use super::Messages::*;


#[derive(Default)]
pub struct
WebSocketServer
{
	clients: HashMap<NetworkPlayerIdentifier, Recipient<JsonMessage>>,
}

impl
WebSocketServer
{
	pub fn 
	send_to_all
	(
		&mut self,
		message: JsonMessage
	)
	{

		let mut to_be_removed = Vec::new();

		for (id, client) in &self.clients
		{
			if let Err(error) = client.try_send(message.to_owned())
			{
				println!("Oh no! Something went wrong with sending data to a player! {:?}", error);
	
				match error
				{
					SendError::Closed(_) => {
						to_be_removed.push(id.to_owned());
					},
					_ => {
						()
					}
				}
			}
		}

		to_be_removed.iter().for_each(|id| { self.clients.remove(id); });
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
		self.subscribe_system_async::<InternalGameListUpdateMessage>(context);
		self.subscribe_system_async::<InternalGameUpdateMessage>(context);
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
Handler<InternalGameListUpdateMessage> 
for 
WebSocketServer 
{
	type Result = MessageResult<InternalGameListUpdateMessage>;

	fn handle(
		&mut self, 
		msg: InternalGameListUpdateMessage, 
		_ctx: &mut Self::Context
	) 
	-> Self::Result 
	{
		// Deconstruct the internal message
		let InternalGameListUpdateMessage(games_list) = msg;

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
Handler<InternalGameUpdateMessage>
for
WebSocketServer
{
	type Result = MessageResult<InternalGameUpdateMessage>;

	fn handle(
		&mut self, 
		msg: InternalGameUpdateMessage, 
		_ctx: &mut Self::Context
	) 
	-> Self::Result 
	{
		// Deconstruct the internal message
		let InternalGameUpdateMessage(game_state, player_list) = msg;

		// Send updated game state to relevant clients
		self.send_game_message(
			JsonMessage(serde_json::to_string(&game_state).unwrap(), None),
			player_list
		);

		// See above
		MessageResult(())
	}
}