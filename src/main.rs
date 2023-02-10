mod board;
mod game;
mod color;
mod server;
mod messages;

use std::sync::Mutex;

use actix_web::{web::{Data}, App, HttpServer};
use server::Server::SudokuServer;

use crate::server::Endpoints::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.app_data(Data::new(Mutex::new(SudokuServer::new())))
			.service(register)
			.service(create_game)
			.service(get_games_list)
			.service(join_game)
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}