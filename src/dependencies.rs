use std::process::Command;

const DEPENDENCIES: &'static [&'static str] = &[
    "plank",
    "xrandr",
    "xdotool",
    "dconf"
];

pub fn startup_check() -> (bool, Vec<&'static str>) {

    let mut missing = Vec::<&'static str>::new();

    for dep in DEPENDENCIES {
        if !check(dep) {
            missing.push(dep);
        }
    }

    (missing.len() == 0, missing)

}

pub fn check(dep: &str) -> bool {

    let output = Command::new("which").arg(dep).output().unwrap();
    output.status.success()

}
