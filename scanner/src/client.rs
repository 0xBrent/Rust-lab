use std::{io::{stdin, Read, Write}, net::TcpStream};

pub fn client_stream() {
  if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8080") {
    println!("Enter msg to send to localhost:8080");
    let mut client_msg = String::new();
    let _buf = [0; 1024];
    stdin()
      .read_line(&mut client_msg)
      .expect("INPUT ERROR");
    stream
      .write(client_msg.as_bytes())
      .unwrap();  
  }
  }