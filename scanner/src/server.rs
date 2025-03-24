use std::fmt::Debug;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::io::*;
use std::ptr::addr_of;
use axum::{extract, Json, ServiceExt};
use axum::routing::{post, trace};
use tokio::*;
use axum::{body::Bytes,extract::{MatchedPath, ConnectInfo},
  http::{HeaderMap, Request},
  response::{Html, Response},
  routing::get,
  Router};
use serde::Deserialize;
use tower_http::{classify::ServerErrorsFailureClass, trace::{TraceLayer, DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse}};
use tracing::{debug_span, info_span, Span, instrument, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing_subscriber::fmt::layer;

#[tokio::main]
pub async fn a_webpage() {
  // Tracing for log purposes

  // application with routes defined
  let app = axum::Router::new()
    .layer(TraceLayer::new_for_http())
    .route("/home", get(|| async {"Rustacean typ shi"}))
    .route("/", get(root)) 
    .route("/json", get(json_data));
    
  // LISTENER
  let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
  tracing::debug!("listening on {}", listener.local_addr().unwrap());
  axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
  
}


async fn root(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
  dbg!(format!("hello {addr}"))
  
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
    println!("SOCKET_INFO: {:?}", stream);
    read_client = stream.read(&mut buf).expect("error");
    println!("Amount of bytes sent: {}", read_client);
    stdout().write(&buf[0..read_client]).unwrap();
  }
}