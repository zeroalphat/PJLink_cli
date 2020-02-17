use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::str::from_utf8;
use std::net::ToSocketAddrs;

fn main() {
    //command argument
    let args: Vec<String> = std::env::args().collect();
    println!("{} {}", &args[1], &args[2]);
    let mut target_address = "192.168.30.40";
    let mut target_port = "4352";
    let command = &args[1];
    let transmission_parameters = &args[2];

    match TcpStream::connect("192.168.30.40:4352") {
        Ok(mut stream) => {
            println!("Successfully connected to server");
            let msg = b"%1POWR 1\r";
            let mut data = [0 as u8; 9];
            
            stream.read_exact(&mut data);
            let negotiation = from_utf8(&data).unwrap();
            println!("{}", negotiation);
            send_command(stream, command, transmission_parameters);
            //stream.write(msg).unwrap();
            //stream.read_exact(&mut data);
            //let result = from_utf8(&data).unwrap();
            //println!("{}", result);
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

fn send_command(mut stream: TcpStream, command: &str, transmission_parameters: &str) {
    let command_header = "%1";
    let separater = " ";
    let terminal_symbol = "\r";
    let send_command = command_header.to_owned() + command + separater + transmission_parameters + terminal_symbol;
    println!("{}", send_command);
    let mut recv_data = [0 as u8; 9];
    stream.write(send_command.as_bytes()).unwrap();
    // recv_data 
    stream.read_exact(&mut recv_data);
    let result = from_utf8(&recv_data).unwrap();
    println!("{}", result);
}
