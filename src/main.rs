mod board;
mod game;
mod color;
mod server;
mod messages;

use std::sync::{Mutex, Arc};

use actix_files;
use actix_web::{web::{Data}, App, HttpServer};
use server::Server::SudokuServer;

use crate::server::Endpoints::*;


use actix_files::NamedFile;
use actix_web::{web, HttpRequest, Result};
use std::path::PathBuf;

async fn index(_req: HttpRequest) -> Result<NamedFile> {
	let path: PathBuf = "./static/index.html".parse().unwrap();
	Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.app_data(Data::new(Mutex::new(SudokuServer::new())))
			.service(
				web::scope("/app")
					.service(register)
					.service(create_game)
					.service(get_games_list)
					.service(join_game)
			)
			.service(actix_files::Files::new("/", "./static").show_files_listing().index_file("index.html"))
	})
	.workers(1)
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}