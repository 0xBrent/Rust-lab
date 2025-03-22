use std::{io::{stdin, Read, Write}, net::TcpStream};

pub fn client_stream() {
  let mut buf = [0; 1024];
  let client_msg = 0;
  let mut client_tcp = TcpStream::connect("127.0.0.1:80").unwrap();
  let reader = client_tcp.write_all(b"Hello TCP").unwrap();
  println!("MSG SENT: {:?}", reader); 
  }