use std::{io::{stdin, Read, Write}, net::TcpStream};

pub fn client_stream() {
  
  let mut client_tcp = TcpStream::connect("127.0.0.1:8080").unwrap();
  let buf = [0; 10];
  let client_msg = client_tcp.write("hello, tcp".as_bytes());}