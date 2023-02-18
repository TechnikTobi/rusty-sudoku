use std::sync::Mutex;

use actix_web::get;
use actix_web::web;
use actix_web::Responder;

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