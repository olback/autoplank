use bincode;
use std::{
    net::Shutdown,
    os::unix::net::UnixStream,
    io::Write,
};
use crate::threads::{
    SocketAction,
    SocketMessage,
    SOCKET_ADDR
};

pub fn rescan() {

    let mut socket = match UnixStream::connect(SOCKET_ADDR) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    let msg = SocketMessage {
        action: SocketAction::RefreshMonitors
    };
    let data = bincode::serialize::<SocketMessage>(&msg).unwrap();
    socket.write(&data).unwrap();
    socket.shutdown(Shutdown::Both).unwrap();

    println!("Rescanning started");

    std::process::exit(0);

}
