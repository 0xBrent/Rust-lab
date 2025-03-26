use std::net::TcpStream;
use std::io::{self, Read, Write};
mod client;
use clap::arg;
pub use client::client_stream;
mod server;
pub use server::server;





// Old client function. Not used
pub fn client_function() {
    let mut client_msg = String::new();
    let client_tcp = TcpStream::connect("127.0.0.1:8080");
    let mut buf = [0; 512];
    for mut stream in client_tcp {
        stream.read(&mut buf).expect("Read error");
        let user_input = io::stdin().read_line(&mut client_msg).unwrap();
        stream.write_all(&user_input.to_be_bytes()).expect("Write error");
        println!("CLIENT MSG: {client_msg:?} stream: {stream:?}")
    } 
}

fn main() {
  let cmd = clap::Command::new("scanner")
    .args(&[
      arg!(-l --listener "Listens on localhost:8080"),
      arg!(-s --socket_connect "Connects to localhost:8080"),
      arg!(-c --count "Count of logical cores in CPU"),
      arg!(-a --axum_server "Start a webserver on localhost:8080")
    ])
    .arg_required_else_help(true)
    .get_matches();

  if cmd.contains_id("listener") {
    println!("Check localhost:8080");
    server::server();
  }

  if cmd.contains_id("socket_connect") {
    println!("Connecting to 8080 on localhost");
    client::client_stream();
  }

  if cmd.contains_id("count") {
    println!("Expiriment using num_cpus crate");
    println!("Number of logical cores is {}", num_cpus::get());
  }

  if cmd.contains_id("axum_server") {
    println!("Web server started on 8080");
    server::a_webpage();
  }

}






// fn old_main() {
//   // Define menu options
//   #[derive(Debug)]
//   struct Menu { 
//     tcp_listener: u32,
//     tcp_connect: u32,
//     cpu_count: u32,
//     axum_webserver: u32,
//   }
//   let main_menu = Menu { 
//     tcp_listener: 1, 
//     tcp_connect: 2, 
//     cpu_count: 3, 
//     axum_webserver: 4
//   };
  
// // print menu options and get user input
//   println!("{:#?}", main_menu);
//   println!("Enter option:");
//   let mut input_int = String::new();
//   io::stdin()
//     .read_line(&mut input_int)
//     .expect("INPUT ERROR");
//   let input:u32 = input_int.trim().parse().expect("Please type a number!");
//   // Options
//   if input == 1 {
//     println!("Check localhost:8080");
//     server::server();
//   } else if input == 2 {
//       if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8080") {
//         println!("Enter msg to send to localhost:8080");
//         let mut client_msg = String::new();
//         let _buf = [0; 1024];
//         io::stdin()
//           .read_line(&mut client_msg)
//           .expect("INPUT ERROR");
//         stream
//           .write(client_msg.as_bytes())
//           .unwrap();  
//       }
//       } else if input == 3 {
//           println!("Expiriment using num_cpus crate");
//           println!("Number of logical cores is {}", num_cpus::get());
//       } else if input == 4 {
//         println!("Web server started on 8080");
//         server::a_webpage();
//       }
//     }
