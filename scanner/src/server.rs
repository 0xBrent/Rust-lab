use std::net::{TcpListener,TcpStream};
use std::io::*;
use axum::{extract, Json};
use axum::routing::post;
use tokio::*;
use axum::{Router, response::Form, routing::get};
use serde::Deserialize;

#[tokio::main]
pub async fn a_webpage() {
  // application with single route
  let app = axum::Router::new()
    .route("/home", get(|| async {"Rustacean typ shi"}))
    .route("/", get(|| async {"Hello, Root"})) 
    .route("/json", get(json_data));
  // LISTENER
  let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
  axum::serve(listener, app).await.unwrap();
  
}

async fn json_data() -> Json<Vec<String>> {
  Json(vec!["Hello".to_owned(), "Rustacean".to_owned()])

}



pub fn server() {
  let site = TcpListener::bind("127.0.0.1:8080").unwrap();
  let mut buf = [0; 1024];
  let mut read_client = 0;
  for stream in site.incoming() {
    let mut stream: TcpStream = stream.unwrap();
    println!("SERVER_CONNECT: {:?}", stream);
    read_client = stream.read(&mut buf).expect("error");
    println!("bytes sent {}", read_client);
    break
}
  stdout().write(&buf[0..read_client]).unwrap();
  

}