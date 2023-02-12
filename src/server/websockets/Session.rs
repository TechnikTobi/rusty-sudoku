use actix::{fut, prelude::*};
use actix_broker::BrokerIssue;
use actix_web::web;
use actix_web_actors::ws;

use std::sync::Mutex;

use crate::{server::Server::SudokuServer, messages::{base::NetworkPlayerIdentifier::NetworkPlayerIdentifier, incoming::GameCreationRequest::GameCreationRequest}, game::player::PlayerID::PlayerID};

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct ChatMessage(pub String);

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct SendMessage(pub String, pub usize, pub String);

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct NewGameMessage(pub GameCreationRequest);

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
		println!("{:?}", web::Json(self.server.as_ref().unwrap().lock().unwrap().generate_games_list_response()));
		println!("Started a WebsocketSession!");
	}

	fn stopped
	(
		&mut self, 
		_ctx: &mut Self::Context
	) 
	{
		println!("{:?}", web::Json(self.server.as_ref().unwrap().lock().unwrap().generate_games_list_response()));
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
	send_msg
	(
		&self, 
		msg: &str
	) 
	{
		let content = format!(
			"{}: {msg}",
			"anon".to_string(),
		);

		let msg = SendMessage("self.room.clone()".to_string(), 1, content);

		self.issue_system_async(msg);
	}
}

impl 
Handler<ChatMessage>
for 
WebsocketSession
{
	type Result = ();

	fn handle
	(
		&mut self, 
		message: ChatMessage, 
		context: &mut Self::Context
	) 
	{
		context.text(message.0);
	}
}

impl
Handler<NewGameMessage>
for
WebsocketSession
{
	type Result = ();

	fn
	handle
	(
		&mut self,
		message: NewGameMessage,
		context: &mut Self::Context
	)
	{
		let request = message.0;
		self.server.as_ref().unwrap().lock().unwrap().get_mut_game_controller_manager().create_game(
			PlayerID::from_network(request.get_player_id()), 
			request.get_game_name().clone(), 
			request.get_difficulty().clone()
		);
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
				}
				else
				{
					println!("{:?}", msg);
				}

				self.send_msg(msg);
			}
			ws::Message::Close(reason) => {
				context.close(reason);
				context.stop();
			}
			_ => {}
		}
	}
}