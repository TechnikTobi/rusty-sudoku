use actix::{prelude::*};
use actix_broker::{BrokerIssue, BrokerSubscribe};
use actix_web::web;
use actix_web_actors::ws;

use std::{sync::Mutex, collections::HashMap};

use crate::{server::Server::SudokuServer, messages::{base::{NetworkPlayerIdentifier::NetworkPlayerIdentifier, NetworkGameIdentifier::NetworkGameIdentifier}, incoming::GameCreationRequest::GameCreationRequest, outgoing::GamesListResponse::GamesListResponse}, game::player::PlayerID::PlayerID};

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct GameCreationRequestMessage(pub GameCreationRequest);

#[derive(Clone, Message, Debug)]
#[rtype(result = "()")]
pub struct GamesListMessage(pub GamesListResponse);

#[derive(Clone, Message, Debug)]
#[rtype(result = "()")]
pub struct GameUpdateMessage();

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

	pub fn
	send_games_list_message
	(
		&self
	)
	{
		let message = GamesListMessage(
			self.server.as_ref().unwrap().lock().unwrap().generate_games_list_response()
		);

		self.issue_system_async(message);
	}
}

// impl 
// Handler<ChatMessage>
// for 
// WebsocketSession
// {
// 	type Result = ();

// 	fn handle
// 	(
// 		&mut self, 
// 		message: ChatMessage, 
// 		context: &mut Self::Context
// 	) 
// 	{
// 		context.text(message.0);
// 	}
// }

// impl
// Handler<GameCreationRequestMessage>
// for
// WebsocketSession
// {
// 	type Result = ();

// 	fn
// 	handle
// 	(
// 		&mut self,
// 		message: GameCreationRequestMessage,
// 		context: &mut Self::Context
// 	)
// 	{
// 		let request = message.0;
// 		self.server.as_ref().unwrap().lock().unwrap().get_mut_game_controller_manager().create_game(
// 			PlayerID::from_network(request.get_player_id()), 
// 			request.get_game_name().clone(), 
// 			request.get_difficulty().clone()
// 		);
// 	}
// }

impl
Handler<GamesListMessage>
for
WebsocketSession
{
	type Result = ();

	fn
	handle
	(
		&mut self,
		message: GamesListMessage,
		context: &mut Self::Context
	)
	{
		context.text(serde_json::to_string(&message.0).unwrap());
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
		message: Result<ws::Message, ws::ProtocolError>, 
		context: &mut Self::Context
	) 
	{
		let msg = match message 
		{
			Err(_) => {
				context.stop();
				return;
			}
			Ok(msg) => msg,
		};

		println!("WEBSOCKET MESSAGE {:?}", msg);

		match msg 
		{
			ws::Message::Text(text) => {

				let msg = text.trim();


				if let Ok(request) = serde_json::from_str::<GameCreationRequest>(msg)
				{
					self.server.as_ref().unwrap().lock().unwrap().get_mut_game_controller_manager().create_game(
						PlayerID::from_network(request.get_player_id()), 
						request.get_game_name().clone(), 
						request.get_difficulty().clone()
					);

					self.send_games_list_message();
				}
				else
				{
					println!("{:?}", msg);
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
	clients: HashMap<NetworkPlayerIdentifier, Recipient<GamesListMessage>>,
	// games_clients: HashMap<NetworkGameIdentifier, Vec<Recipient<GameUpdateMessage>>>,
}

impl
WebSocketServer
{

	pub fn
	add_client
	(
		&mut self,
		client: Recipient<GamesListMessage>
	)
	{

	}

	pub fn 
	send_to_all
	(
		&self,
		message: GamesListMessage
	)
	{
		for (_, client) in &self.clients
		{
			client.try_send(message.to_owned());
		}
	}
}

impl 
Actor 
for 
WebSocketServer 
{
	type Context = Context<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {
		// self.subscribe_system_async::<GamesListMessage>(ctx);
		// self.subscribe_system_async::<GameUpdateMessage>(ctx);
		self.subscribe_system_async::<GameCreationRequestMessage>(ctx);
	}
}

impl 
Handler<GamesListMessage> 
for 
WebSocketServer 
{
	type Result = MessageResult<GamesListMessage>;

	fn handle(&mut self, msg: GamesListMessage, _ctx: &mut Self::Context) -> Self::Result {
		let JoinRoom(room_name, client_name, client) = msg;

		let id = self.add_client_to_room(&room_name, None, client);
		let join_msg = format!(
			"{} joined {room_name}",
			client_name.unwrap_or_else(|| "anon".to_string()),
		);

		self.send_chat_message(&room_name, &join_msg, id);
		MessageResult(id)
	}
}

impl SystemService for WebSocketServer {}
impl Supervised for WebSocketServer {}