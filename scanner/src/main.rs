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
  let mut input_int = String::new();
  println!("Enter option:");
  io::stdin().read_line(&mut input_int);
  let input:u32 = input_int.trim().parse().expect("Please type a number!");
  if input == 1 {
    server::server();
  } else if input == 2 {
    println!("Enter msg: ");
    let mut client_msg = String::new();
    let buf = [0; 1024];
    io::stdin().read_line(&mut client_msg);
    let write = TcpStream::connect("127.0.0.1:8080").unwrap().write_all(&buf);
    TcpListener::bind("127.0.0.1:8080").unwrap().incoming();
    
    println!("MSG SENT: {:?}", write); 
    
    // client::client_stream();
  }
}
