use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};

use crate::board::*;

#[get("/")]
async fn hello() -> impl Responder {
	HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
	HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
	HttpResponse::Ok().body("Hey there!")
}

#[derive(Serialize, Deserialize)]
struct
SomeData
{
	data1: String,
	data2: String
}

#[post("/register")]
async fn
register
(
	request_body: web::Json<SomeData>
)
-> impl Responder
{
	// HttpResponse::Ok().body(format!("hi {}", request_body))
	return web::Json(SomeData{ data1: request_body.data2.clone(), data2: request_body.data1.clone() });
}