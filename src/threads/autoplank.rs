use std::sync::{Arc, Mutex};
use super::super::{
    plank::Plank,
    mouse_location::MouseLocation,
    monitor::Monitor
};

pub fn autoplank(m: Arc<Mutex<Vec<Monitor>>>) {

    let mut p = Plank::new();

    loop {

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

        std::thread::sleep(std::time::Duration::from_millis(500));

    }

}
