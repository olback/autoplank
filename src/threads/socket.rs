use super::super::{
    monitor::Monitor
};
use std::{
    time,
    thread,
    os::unix::net::{
        UnixListener
    },
    io::{
        self,
        Read,
        BufReader,
        Write
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
pub enum SocketMessage {
    Ok(Vec<Monitor>),
    Err(String),
    RefreshMonitors
}

pub fn socket(m: Arc<Mutex<Vec<Monitor>>>) -> Result<(), io::Error> {

    if Path::new(SOCKET_ADDR).exists() {
        fs::remove_file(SOCKET_ADDR)?;
    }

    let listener = UnixListener::bind(SOCKET_ADDR)?;

    for stream in listener.incoming() {

        let mut s = stream?;

        thread::sleep(time::Duration::from_millis(10));

        let mut buf = BufReader::new(&s);
        let mut data = [0u8; 32];
        let len = buf.read(&mut data)?;

        let msg: SocketMessage = match bincode::deserialize(&data[0..len]) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        match msg {

            SocketMessage::RefreshMonitors => {

                println!("=> Scanning for monitors...");

                let monitors = padlock::mutex_lock(&m, |lock| {
                    *lock = Monitor::get_all();
                    lock.clone()
                });

                let data = bincode::serialize(&SocketMessage::Ok(monitors)).unwrap();
                s.write(&data[..])?;

            },

            _ => eprintln!("Received invalid data")

        };

    }

    Ok(())

}
