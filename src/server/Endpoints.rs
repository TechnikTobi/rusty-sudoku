use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::game::player::PlayerID::PlayerID;
use crate::messages::incoming::GameCreationRequest::GameCreationRequest;
use crate::messages::outgoing::PlayerRegistrationResponse::PlayerRegistrationResponse;
use crate::messages::incoming::PlayerRegistrationRequest::PlayerRegistrationRequest;

use crate::server::Server::SudokuServer;


#[post("/register")]
async fn
register
(
	server: web::Data<Mutex<SudokuServer>>,
	request_body: web::Json<PlayerRegistrationRequest>,
)
-> impl Responder
{
	let new_player_id = server
		.lock()
		.unwrap()
		.get_mut_player_manager()
		.add_player(request_body.get_player_name().clone())
		.to_network();

	return web::Json(PlayerRegistrationResponse::new(new_player_id));
}

#[post("/createGame")]
async fn
create_game
(
	server: web::Data<Mutex<SudokuServer>>,
	request_body: web::Json<GameCreationRequest>,
)
-> impl Responder
{
	let new_game_id = server.lock().unwrap()
		.get_mut_game_controller_manager()
		.create_game(
			PlayerID::from_network(request_body.get_player_id()), 
			request_body.get_game_name().clone(), 
			request_body.get_difficulty().clone()
		)
		.to_network();

	return web::Json(server.lock().unwrap().generate_games_list_response());
}

#[get("/getGamesList")]
async fn
get_games_list
(
	server: web::Data<Mutex<SudokuServer>>
)
-> impl Responder
{
	return web::Json(server.lock().unwrap().generate_games_list_response());
}

