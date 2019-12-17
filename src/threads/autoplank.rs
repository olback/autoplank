use std::sync::{Arc, Mutex};
use super::super::{
    plank::Plank,
    mouse_location::MouseLocation,
    monitor::Monitor
};
use nix::{
    sys::signal,
    unistd::Pid
};

pub fn autoplank(m: Arc<Mutex<Vec<Monitor>>>, rate: u64) {

    let plank_pid = Plank::get_pid();
    if plank_pid.is_some() {

        match signal::kill(Pid::from_raw(plank_pid.unwrap()), signal::Signal::SIGINT) {
            Ok(_) => {
                println!("Plank killed");
            },
            Err(e) => eprintln!("Error: {:#?}", e)
        }

    }

    let mut p = Plank::new();

    loop {

        // Sleep before locking the mutex to minimize the lock-time.
        std::thread::sleep(std::time::Duration::from_millis(rate));

        let ml = MouseLocation::get();
        let monitors = &*m.lock().unwrap();

        for monitor in monitors {

            if monitor.mouse_here(&ml) {

                if p.set_monitor(&monitor.name) {
                    // Plank needs to be restarted if the change needs to happen immediately.
                    p.restart();
                }

            }

        }

    }

}
