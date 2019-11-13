mod mouse_location;
mod monitor;
mod plank;
mod dependencies;
use mouse_location::MouseLocation;
use monitor::Monitor;
use plank::Plank;

fn main() {

    println!("Autoplank");

    let deps = dependencies::check();

    if !deps.0 {
        eprintln!("Missing dependencies:");
        for dep in deps.1 {
            eprintln!(" - {}", dep);
        }
        std::process::exit(1);
    }

    let monitors = Monitor::get_all();
    let mut p = Plank::new();

    loop {

        let ml = MouseLocation::get();

        for monitor in &monitors {

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
