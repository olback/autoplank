use super::super::{
    monitor::Monitor
};
use std::{
    os::unix::net::{
        UnixListener
    },
    io::{
        Read,
        BufReader
    },
    sync::{
        Arc,
        Mutex
    },
    path::Path,
    fs
};
use serde::{Serialize, Deserialize};
use bincode;

pub const SOCKET_ADDR: &str = "/tmp/autoplank.sock";

#[derive(Debug, Serialize, Deserialize)]
pub struct SocketMessage {
    pub action: SocketAction
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SocketAction {
    RefreshMonitors = 1
}

pub fn socket(m: Arc<Mutex<Vec<Monitor>>>) {

    if Path::new(SOCKET_ADDR).exists() {
        fs::remove_file(SOCKET_ADDR).unwrap();
    }
    let listener = UnixListener::bind(SOCKET_ADDR).unwrap();

    for stream in listener.incoming() {

        match stream {

            Ok(stream) => {

                std::thread::sleep(std::time::Duration::from_millis(10));

                let mut buf = BufReader::new(stream);
                let mut data = Vec::<u8>::new();
                buf.read_to_end(&mut data).unwrap();
                let msg: SocketMessage = match bincode::deserialize(&data) {
                    Ok(m) => m,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                };

                match msg.action {

                    SocketAction::RefreshMonitors => {
                        println!("=> Rescanning...");
                        let mut monitors = m.lock().unwrap();
                        *monitors = Monitor::get_all();
                    }

                };

            },
            Err(e) => {

                eprintln!("Socket error: {}", e);

            }

        }

    }

}
