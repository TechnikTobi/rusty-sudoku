mod board;
mod game;
mod color;
mod server;
mod messages;

use std::sync::{Mutex, Arc};
use std::path::PathBuf;

use actix_files;
use actix_web::{web::{Data}, App, HttpServer};
use actix_web::{middleware::Logger, Error, Responder};
use actix_web_actors::ws;
use actix_files::NamedFile;
use actix_web::{web, HttpRequest, Result};

use server::Server::SudokuServer;
use server::Endpoints::*;
use server::websockets::Session::WebsocketSession;

async fn 
index
(
	_req: HttpRequest
) 
-> Result<NamedFile> 
{
	let path: PathBuf = "./static/index.html".parse().unwrap();
	Ok(NamedFile::open(path)?)
}


async fn
websocket
(
	req: HttpRequest, 
	stream: web::Payload,
	app_data: web::Data<Mutex<SudokuServer>>
) 
-> Result<impl Responder, Error> 
{
	ws::start(WebsocketSession::new(Some(app_data)), &req, stream)
}


#[actix_web::main]
async fn 
main
() 
-> std::io::Result<()> 
{
	HttpServer::new(|| {
		App::new()
			.app_data(Data::new(Mutex::new(SudokuServer::new())))
			.service(
				web::scope("/app")
					.service(get_games_list)
			)
			.service(web::resource("/websocket").to(websocket))
			.service(actix_files::Files::new("/", "./static").show_files_listing().index_file("index.html"))
	})
	.workers(1)
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}