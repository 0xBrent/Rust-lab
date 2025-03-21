use std::net::{TcpListener,TcpStream};
use std::io::{self, stdout, Read, Write};
mod client;
pub use client::client_stream;
mod server;
pub use server::server;






pub fn client_function() {
    let mut client_msg = String::new();
    let client_tcp = TcpStream::connect("127.0.0.1:8080");
    let mut buf = [0; 512];
    for mut stream in client_tcp {
        stream.read(&mut buf);
        let user_input = io::stdin().read_line(&mut client_msg).unwrap();
        stream.write_all(&user_input.to_be_bytes());
        println!("CLIENT MSG: {client_msg:?} stream: {stream:?}")
    } 
}

fn main() {
    let mut input = String::new();

  println!("Enter: ");
  match io::stdin().read_line(&mut input) {
    Ok((_input)) => {
        server::server();
        client::client_stream();
        println!("tcp started on 8080")
    }
    Err(e) => {
      println!("Error with input try again {}", e);
      
    }
    _ => {
      println!("Unexpected.")
    }
  }
}
