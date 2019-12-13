use std::process::Command;
use super::mouse_location::MouseLocation;
use regex::Regex;

#[derive(Debug)]
pub struct Offset {
    pub x: u32,
    pub y: u32
}

#[derive(Debug)]
pub struct Resolution {
    pub width: u32,
    pub height: u32
}

#[derive(Debug)]
pub struct Monitor {
    pub name: String,
    pub resolution: Resolution,
    pub offset: Offset,
    pub primary: bool
}

impl Monitor {

    pub fn get_all() -> Vec<Monitor> {

        let mut monitors = Vec::<Monitor>::new();
        let re = Regex::new("[\\d]{1,9}[x][\\d]{1,9}[\\+][\\d]{1,9}[\\+][\\d]{1,9}").unwrap();

        let output = Command::new("xrandr").output().unwrap();
        let raw_str = String::from_utf8_lossy(&output.stdout);
        let all_lines: Vec<&str> = raw_str.trim().split("\n").collect();
        let mut monitor_strings = Vec::<String>::new();

        for line in &all_lines {
            if line.contains(" connected ") && re.is_match(line) {
                monitor_strings.push(String::from(*line));
            } else if line.contains(" connected ") {
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                println!("=> Ignoring {}", parts[0]);
            }
        }

        for ms in &monitor_strings {

            let parts: Vec<&str> = ms.trim().split_whitespace().collect();

            let name = String::from(parts[0]);
            let primary = match parts[2] { "primary" => true, _ => false };
            let (resolution, offset) = Self::parse_res_offset(match primary { true => parts[3], false => parts[2] });

            monitors.push(Monitor {
                name: name,
                resolution: resolution,
                offset: offset,
                primary: primary
            });

        }

        for m in &monitors {
            println!("=> Found {}", m);
        }

        monitors

    }

    // Expects format like "1920x1080+1920+0"
    fn parse_res_offset(s: &str) -> (Resolution, Offset) {

        let mut values = s.split(|c| c == 'x' || c == '+');

        (
            Resolution {
                width: values.next().unwrap().parse().unwrap(),
                height: values.next().unwrap().parse().unwrap()
            },
            Offset {
                x: values.next().unwrap().parse().unwrap(),
                y: values.next().unwrap().parse().unwrap()
            }
        )

    }

    pub fn mouse_here(&self, mouse_location: &MouseLocation) -> bool {

        return mouse_location.x >= self.offset.x && mouse_location.x < self.offset.x + &self.resolution.width

    }

}

impl std::fmt::Display for Monitor {

    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.primary {
            write!(fmt, "{} primary {}x{}+{}+{}", self.name, self.resolution.width, self.resolution.height, self.offset.x, self.offset.y)
        } else {
            write!(fmt, "{} {}x{}+{}+{}", self.name, self.resolution.width, self.resolution.height, self.offset.x, self.offset.y)
        }
    }

}
