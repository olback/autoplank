use std::process::Command;

const DEPENDENCIES: &'static [&'static str] = &["plank", "xrandr", "xdotool", "dconf"];

pub fn check() -> (bool, Vec<&'static str>) {

    let mut missing = Vec::<&'static str>::new();

    for dep in DEPENDENCIES {
        let output = Command::new("which").arg(dep).output().unwrap();
        if !output.status.success() {
            missing.push(dep);
        }
    }

    (missing.len() == 0, missing)

}
