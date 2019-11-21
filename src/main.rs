mod mouse_location;
mod monitor;
mod plank;
mod dependencies;
mod threads;

use monitor::Monitor;
use threads::{
    SOCKET_ADDR,
    SocketAction,
    SocketMessage
};
use std::{
    net::Shutdown,
    os::unix::net::UnixStream,
    io::Write,
    sync::{
        Arc,
        Mutex
    }
};
use clap::{self, load_yaml, crate_version};
use bincode;

fn main() {

    let yml = load_yaml!("../cli.yml");
    let matches = clap::App::from(yml).version(crate_version!()).get_matches();

    if matches.is_present("rescan") {

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

    println!("Autoplank");

    // Make sure all dependencies are installed
    let deps = dependencies::check();
    if !deps.0 {
        eprintln!("Missing dependencies:");
        for dep in deps.1 {
            eprintln!(" - {}", dep);
        }
        std::process::exit(1);
    }

    // Keep track of monitors
    let monitors = Arc::new(Mutex::new(Monitor::get_all()));
    let mut thread_handlers = Vec::<std::thread::JoinHandle<()>>::new();

    // Autoplank thread, fetches mouse location every 500ms
    let autoplank_monitors = Arc::clone(&monitors);
    thread_handlers.push(std::thread::spawn(|| {
        threads::autoplank(autoplank_monitors);
    }));

    // Socket thread
    let socket_monitors = Arc::clone(&monitors);
    thread_handlers.push(std::thread::spawn(|| {
        threads::socket(socket_monitors);
    }));

    for th in thread_handlers {
        th.join().unwrap();
    }

}
