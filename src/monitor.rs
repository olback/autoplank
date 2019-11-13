use std::process::Command;
use super::MouseLocation;

#[derive(Debug)]
pub struct Offset {
    pub x: u32,
    pub y: u32
}

#[derive(Debug)]
pub struct Monitor {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub offset: Offset,
    pub primary: bool
}

impl Monitor {

    pub fn get_all() -> Vec<Monitor> {

        let mut monitors = Vec::<Monitor>::new();

        let output = Command::new("xrandr").output().unwrap();
        let raw_str = String::from_utf8_lossy(&output.stdout);
        let all_lines: Vec<&str> = raw_str.trim().split("\n").collect();
        let mut monitor_strings = Vec::<String>::new();

        for line in &all_lines {
            if line.contains(" connected ") {
                monitor_strings.push(String::from(*line));
            }
        }

        for ms in &monitor_strings {

            let parts: Vec<&str> = ms.trim().split_whitespace().collect();

            let name = String::from(parts[0]);
            let primary = match parts[2] { "primary" => true, _ => false };
            let (width, height, offset) = Self::parse_res_offset(match primary { true => parts[3], false => parts[2] });

            monitors.push(Monitor {
                name: name,
                width: width,
                height: height,
                offset: offset,
                primary: primary
            });

        }

        monitors

    }

    fn parse_res_offset(s: &str) -> (u32, u32, Offset) {

        let mut values = s.split(|c| c == 'x' || c == '+');

        let mut offset = Offset { x: 0, y: 0 };

        let width = values.next().unwrap().parse().unwrap();
        let height = values.next().unwrap().parse().unwrap();
        offset.x = values.next().unwrap().parse().unwrap();
        offset.y = values.next().unwrap().parse().unwrap();

        (width, height, offset)

    }

    pub fn mouse_here(&self, mouse_location: &MouseLocation) -> bool {

        return mouse_location.x >= self.offset.x && mouse_location.x < self.offset.x + &self.width

    }

}


