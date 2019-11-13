use std::process::Command;

#[derive(Debug)]
pub struct MouseLocation {
    pub x: u32,
    pub y: u32,
    pub screen: u8,
    pub window: u32
}

impl MouseLocation {

    pub fn get() -> Self {

        let output = Command::new("xdotool").arg("getmouselocation").output().unwrap();
        let raw_str = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = raw_str.trim().split(" ").collect();

        let mut location = Self {
            x: 0,
            y: 0,
            screen: 0,
            window: 0
        };

        for line in &lines {

            let parts: Vec<&str> = line.split(":").collect();

            match parts[0] {
                "x" => {
                    location.x = parts[1].parse().unwrap();
                },
                "y" => {
                    location.y = parts[1].parse().unwrap();
                },
                "screen" => {
                    location.screen = parts[1].parse().unwrap();
                },
                "window" => {
                    location.window = parts[1].parse().unwrap();
                },
                _ => {
                    // println!("Unknown property");
                }
            }

        }

        location

    }

}
