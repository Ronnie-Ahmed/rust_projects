

use std::net::{TcpStream,TcpListener};
use std::io::{Bytes, Read, Write};
use std::thread;


pub fn Simple_Chat(){
    let listener=TcpListener::bind("127.0.0.1:8080").expect("Error Connecting");
    println!("Server Listening on port 8080");

    for stream in listener.incoming(){
        let stream=stream.expect("Failed to Established Connection");
        println!("New Client Connected");

        thread::spawn(||{
            handle_client(stream)

        });
    }

}

fn handle_client(mut stream:TcpStream){
    let mut buffer=[0;512];

    loop{
        match stream.read(&mut buffer){
            Ok(bytes_read)=>{
                if bytes_read==0{
                    println!("Client Disconnected");
                    break;
                }
                let received_message=String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Received Message: {}",received_message);
                stream.write_all(&buffer[..bytes_read]).expect("Falied");

            }
            Err(err)=>{
                println!("Error Reading the socket: {}",err);
                break;
            }

        }
    }

}