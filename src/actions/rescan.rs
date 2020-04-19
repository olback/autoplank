use bincode;
use std::{
    net::Shutdown,
    os::unix::net::UnixStream,
    io::{self, Read, Write}
};
use crate::threads::{
    SocketMessage,
    SOCKET_ADDR
};

pub fn rescan() -> Result<(), io::Error>{

    // Open socket
    let mut socket = UnixStream::connect(SOCKET_ADDR)?;

    println!("Rescanning started...");

    // Request rescan
    let msg = SocketMessage::RefreshMonitors;
    let data = bincode::serialize::<SocketMessage>(&msg).unwrap();
    socket.write(&data)?;

    // Receive data
    let mut buf = [0u8; 512];
    let len = socket.read(&mut buf)?;
    let data = bincode::deserialize::<SocketMessage>(&buf[0..len]).unwrap();

    // Bye
    socket.shutdown(Shutdown::Both).unwrap();

    let status = match data {

        SocketMessage::Ok(monitors) => {
            for m in monitors {
                println!("Found {}", m.to_string());
            }
            println!("Rescan done!");
            0
        },

        SocketMessage::Err(e) => {
            eprintln!("{}", e);
            1
        },

        _ => {
            eprintln!("Received invalid data");
            1
        }

    };

    std::process::exit(status);

}
