use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::game::GameID::GameID;
use crate::game::player::PlayerID::PlayerID;
use crate::messages::incoming::GameCreationRequest::GameCreationRequest;
use crate::messages::incoming::GameJoinRequest::GameJoinRequest;
use crate::messages::outgoing::PlayerRegistrationResponse::PlayerRegistrationResponse;
use crate::messages::outgoing::IResponse::*;
use crate::messages::incoming::PlayerRegistrationRequest::PlayerRegistrationRequest;

use crate::server::Server::SudokuServer;

#[get("/getGamesList")]
async fn
get_games_list
(
	server: web::Data<Mutex<SudokuServer>>,
)
-> impl Responder
{
	return web::Json(server.lock().unwrap().generate_games_list_response());
}