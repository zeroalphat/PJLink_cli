use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::str::from_utf8;
use std::net::ToSocketAddrs;
use structopt::{clap, StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(name = "joseph")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
struct Opt {
    #[structopt(name = "CMD")]
    command: String,

    #[structopt(name = "parrameter")]
    transmission: String,
}


fn main() {
    //command argument
    let opt = Opt::from_args();
    //println!("{:#?}", &opt);
    
    let command = &opt.command;
    let transmission_parameters = &opt.transmission;
    println!("{} {}", command, transmission_parameters);

    match TcpStream::connect("192.168.30.40:4352") {
        Ok(mut stream) => {
            println!("Successfully connected to server");
            let mut data = [0 as u8; 9];
            
            stream.read_exact(&mut data);
            let negotiation = from_utf8(&data).unwrap();
            println!("{}", negotiation);
            send_command(stream, command, transmission_parameters);
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
