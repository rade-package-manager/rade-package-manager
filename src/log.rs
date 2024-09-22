// SPDX-License-Identifier: MIT
// Copyright (c) 2024 17do
// This software is licensed under the MIT License.

use chrono::{Datelike, Utc};
use colored::Colorize;
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use crate::Package;

#[derive(Serialize, Deserialize)]
struct Time {
    utc_day: String,
    utc_time: String,
}
#[derive(Serialize, Deserialize)]
struct Install {
    install_name: String,
}
#[derive(Serialize, Deserialize)]
struct Info {
    version: String,
    repositry: String,
}
#[derive(Serialize, Deserialize)]
struct Config {
    time: Time,
    install: Install,
    info: Info,
}
#[derive(Serialize, Deserialize)]
struct ConfG {
    time: Time,
    install: i64,
}

pub struct Name<'a> {
    pub basedir: &'a Path,
}

impl<'a> Name<'a> {
    /// set path.
    pub fn new(base: &'a Path) -> Self {
        Name { basedir: base }
    }

    /// create package log files.
    pub fn create(
        &self,
        package: &str,
        _install: &str,
        url: String,
        versi: String,
    ) -> Result<(), Box<dyn Error>> {
        let _name = self.basedir.join(package);
        println!(
            "{} {}",
            "==>".bold().blue(),
            _name.display().to_string().as_str().bold()
        );
        let utc = Utc::now();
        let data = format!("{}-{}-{}", utc.year(), utc.month(), utc.day());
        let time_ = utc.time().format("%H:%M:%S").to_string();
        let config = Config {
            time: Time {
                utc_day: data,
                utc_time: time_,
            },
            install: Install {
                install_name: _install.to_string(),
            },
            info: Info {
                version: versi,
                repositry: url,
            },
        };
        let toml_str = toml::to_string(&config)?;
        let s = fs::read_to_string(Package::rade_home().join("log/status"))?;
        let mut tml: ConfG = toml::from_str(&s)?;
        tml.install += 1;
        fs::write(&_name, toml_str)?;
        let update_str = toml::to_string(&tml)?;
        fs::write(Package::rade_home().join("log/status"), update_str)?;
        Ok(())
    }
    pub fn remove_program(&self, pkgname: &str) {
        let _name = self.basedir.join(pkgname);
        fs::remove_file(_name).expect("Failed to remove log");
    }
}

pub fn new() {
    let ps = home_dir()
        .expect("Failed to get home")
        .join(".comrade/log/status");
    if ps.exists() {
        println!("Would you like to refresh the log?");
        println!("This action will erase all previous logs");
        print!("[y/n] ");
        io::stdout().flush().unwrap();
        let mut _str = String::new();
        io::stdin().read_line(&mut _str).unwrap();
        let _str = _str.trim();
        if ["y", "yes", ""].contains(&_str) {
            println!("{} Start creating log...", ">>>".blue().bold());
            println!("{} Deleting status file...", ">>>".green().bold());
            fs::remove_file(
                home_dir()
                    .expect("Failed to get home dir.")
                    .join(".comrade/log/status"),
            )
            .expect("Failed to remove file");
            let utc = chrono::Utc::now();
            let data = format!("{}-{}-{}", utc.year(), utc.month(), utc.day());
            let time_ = utc.time().format("%H:%M:%S").to_string();
            println!("{} Create log status...", ">>>".green().bold());
            let config: ConfG = ConfG {
                time: Time {
                    utc_day: data,
                    utc_time: time_,
                },
                install: 0,
            };
            let toml = toml::to_string(&config).unwrap();
            fs::write(&ps, toml).unwrap();
            println!("{}", "Log creation completed successfully".bold().cyan());
        } else {
            println!("Clearing the log has been canceled");
            std::process::exit(1);
        }
    } else {
        println!("{} Start creating log...", ">>>".blue().bold());
        let utc = chrono::Utc::now();
        let data = format!("{}-{}-{}", utc.year(), utc.month(), utc.day());
        let time_ = utc.time().format("%H:%M:%S").to_string();
        println!("{} Create log status...", ">>>".green().bold());
        let config: ConfG = ConfG {
            time: Time {
                utc_day: data,
                utc_time: time_,
            },
            install: 0,
        };
        let toml = toml::to_string(&config).unwrap();
        fs::write(&ps, toml).unwrap();
        println!("{}", "Log creation completed successfully".bold().cyan());
    }
}

pub fn status() {
    println!(
        "{} {}",
        ">>>".green().bold(),
        "loading status file...".bold()
    );
    let (install, time_utc, utc_day) = parse_status();

    let mut day_: Vec<&str> = utc_day.split('-').collect();
    let lis = &[
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let day1: usize = day_[1].parse().unwrap();
    day_[1] = lis[day1 - 1];
    println!(
        "Using logs created in {}, {}, {}st, {}",
        day_[0], day_[1], day_[2], time_utc
    );
    println!("Total number of installations {}", install);
}

/// return (install, time_utc, utc_day)
pub fn parse_status() -> (String, String, String) {
    let status_file = match fs::read_to_string(Package::rade_home().join("log/status")) {
        Ok(o) => o,
        Err(e) => {
            eprintln!(
                "{} {}",
                ">>>".red().bold(),
                "Failed to read status file".bold()
            );
            eprintln!("`rade log new` to create new log status.");
            eprintln!("Error code: {}", e);
            std::process::exit(1);
        }
    };

    let toml: ConfG = toml::from_str(&status_file).unwrap();

    (
        toml.install.to_string(),
        toml.time.utc_time,
        toml.time.utc_day,
    )
}
