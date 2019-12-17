use crate::cerbere::Cerbere;
use crate::dependencies;
use std::process::Command;

const SCHEMA: &str = "io.elementary.desktop.cerbere";
const KEY: &str = "monitored-processes";

fn check_gsettings() {

    if !dependencies::check("gsettings") {
        eprintln!("Missing gsettings (libglib2.0-0)");
        std::process::exit(1);
    }

}

fn util(fix: bool) {

    check_gsettings();

    let current = match Command::new("gsettings").args(&["get", SCHEMA, KEY]).output() {
        Ok(v) => {
            match v.status.success() {
                true => String::from_utf8_lossy(&v.stdout).trim().to_string(),
                false => {
                    eprintln!("gsettings did not exix successfully");
                    std::process::exit(1);
                }
            }
        },
        Err(e) => {
            eprintln!("{:#?}", e);
            std::process::exit(1);
        }
    };

    let mut cerbere = Cerbere::from(current);

    if fix {
        cerbere.remove("'plank'");
    } else {
        cerbere.add("'plank'");
    }

    match Command::new("gsettings").args(&["set", SCHEMA, KEY, &cerbere.to_string()]).output() {
        Ok(v) => {
            if !v.status.success() {
                eprintln!("gsettings did not exix successfully");
                std::process::exit(1);
            }

        },
        Err(e) => {
            eprintln!("{:#?}", e);
            std::process::exit(1);
        }
    }

    println!("Success");
    std::process::exit(0);

}

pub fn fix() {

    util(true);

}

pub fn restore() {

    util(false);

}
