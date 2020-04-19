use std::process::{Command, Child, Stdio};
use sysinfo::{ProcessExt, SystemExt};

const DCONF_KEY: &str = "/net/launchpad/plank/docks/dock1/monitor";

pub struct Plank {
    child: Child
}

impl Plank {

    pub fn new() -> Self {
        Self {
            child: Self::spawn()
        }
    }

    pub fn set_monitor(&self, mon: &String) -> bool {

        let output = Command::new("dconf").args(&["read", DCONF_KEY]).output().unwrap();
        let current = String::from_utf8_lossy(&output.stdout);
        let new = format!("'{}'", mon);

        if current.to_string().trim() != new {

            println!("=> Switching to {}", mon);

            let output = Command::new("dconf").args(&["write", DCONF_KEY, &new]).output().unwrap();
            let status = output.status.success();

            if !status {
                eprintln!("Failed to write new value to dconf");
            }

            return status;

        }

        false

    }

    pub fn restart(&mut self) {
        self.kill();
        self.child = Self::spawn();
    }

    fn kill(&mut self) {
        self.child.kill().unwrap();
        self.child.wait().unwrap(); // prevent zombie processes
    }

    fn spawn() -> Child {
        return Command::new("plank")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    }

    pub fn get_pid() -> Option<i32> {

        let mut system = sysinfo::System::new();
        system.refresh_all();

        for (pid, process) in system.get_processes() {
            if process.name() == "plank" {
                return Some(*pid)
            }
        }

        None

    }

}

impl Drop for Plank {

    fn drop(&mut self) {
        self.kill();
    }

}
