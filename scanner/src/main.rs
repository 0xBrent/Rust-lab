use std::net::{TcpListener,TcpStream};
use std::io::{self, stdout, Read, Write};
mod client;
pub use client::client_stream;


pub fn server(mut stream: TcpStream) {
    let mut buf = [0; 512];
    let mut read_client = 0;
    while read_client == 0 {
        read_client = stream.read(&mut buf).unwrap();
        println!("bytes sent {}", read_client);
    
    
    }
    stdout().write(&buf[0..read_client]).unwrap();

}


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
    let site = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in site.incoming() {
        match stream {
            Ok(stream) => {
                client::client_stream();
                server(stream);
                
            }
            Err(_e) => println!("NO SOCKET BIND")

        }
    }
}




// pub fn client_stream() {
  
//     let client_tcp = TcpStream::connect("127.0.0.1:8080");
//     let mut client_msg = b"MIC CHECK MIC CHECK 1 2";
//     for mut stream in client_tcp {
//       stream.write_all(client_msg);
      
  
//     }
  
  
//   }