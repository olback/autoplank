use std::process::{Command, Child, Stdio};

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

            println!("Switching to {}", new);

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
        &self.child.kill();
        self.child = Self::spawn();
    }

    fn spawn() -> Child {
        return Command::new("plank")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    }

}
