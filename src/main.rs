mod mouse_location;
mod monitor;
mod plank;
mod dependencies;
mod threads;
mod cerbere;
mod actions;

use monitor::Monitor;
use std::{
    sync::{
        Arc,
        Mutex
    }
};
use clap::{self, load_yaml, crate_version};

fn main() {

    let yml = load_yaml!("../cli.yml");
    let matches = clap::App::from(yml).version(crate_version!()).get_matches();

    if matches.is_present("rescan") {
        actions::rescan();
    } else if matches.is_present("elementary-fix") {
        actions::elementary::fix()
    } else if matches.is_present("elementary-restore") {
        actions::elementary::restore();
    }

    let polling_rate = match matches.value_of("polling-rate") {
        Some(v) => v.parse::<u64>().expect("Error parsing polling rate"),
        None => 500
    };

    println!("Autoplank");
    println!("=> Polling rate set to {}ms", polling_rate);

    // Make sure all dependencies are installed
    let deps = dependencies::startup_check();
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
    thread_handlers.push(std::thread::spawn(move || {
        threads::autoplank(autoplank_monitors, polling_rate);
    }));

    // Socket thread
    let socket_monitors = Arc::clone(&monitors);
    thread_handlers.push(std::thread::spawn(move || {
        threads::socket(socket_monitors);
    }));

    for th in thread_handlers {
        th.join().unwrap();
    }

}
