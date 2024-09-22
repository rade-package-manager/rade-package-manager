// SPDX-License-Identifier: MIT
// Copyright (c) 2024 17do
// This software is licensed under the MIT License.

use crate::Package;
use colored::*;
use dirs::home_dir;
use serde::Deserialize;
use std::ffi::OsStr;
use std::fs;

#[derive(Debug, Deserialize)]
struct Time {
    #[allow(unused)]
    utc_day: String,
    #[allow(unused)]
    utc_time: String,
}
#[derive(Deserialize, Debug)]
struct Install {
    install_name: String,
}
#[derive(Deserialize, Debug)]
struct Info {
    version: String,
    repositry: String,
}
#[derive(Deserialize, Debug)]
struct Config {
    #[allow(unused)]
    time: Time,
    install: Install,
    info: Info,
}

pub fn program_exists(packagename: &str) -> bool {
    let dir_path = dirs::home_dir()
        .expect("Failed to get home directory")
        .join(".comrade/log/install");
    let dir = match fs::read_dir(&dir_path) {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!(
                "{}{}Please report this issue to the comrade package manager repositoryError code: {}",
                ">>> ".red().bold(),
                "Failed to read log files.\n".bold(),
                e
            );
            return false;
        }
    };
    let mut found: bool = false;
    let mut ret: bool = false;
    for entry in dir.flatten() {
        if entry.file_name() == <&str as AsRef<OsStr>>::as_ref(&packagename) {
            found = true;
            ret = true;
            break;
        }
    }

    if !found {
        println!(
            "{} {}{}",
            ">>>".red().bold(),
            "Program not found: ".bold(),
            &packagename.bold()
        );
        ret = false;
    }
    ret
}
impl Package {
    /// # log_parse
    /// log paeser.
    /// ## usage
    /// `Package::log_parse(_package_name_)`
    ///
    /// ## return
    /// this function is return the taple
    /// `(String, String, String)`
    /// return list is
    /// `(executable_name, repositry_url, package_version)`
    pub fn log_parse(packagename: &str) -> (String, String, String) {
        let installdir = home_dir()
            .expect("Failed to get home dir")
            .join(".comrade/log/install/");
        let base = format!("{}{}", installdir.display(), &packagename);
        println!("{} {}", "==>".bold().blue(), &base.as_str().bold());
        let toml_file = match fs::read_to_string(base) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("Failed to read package log.");
                eprintln!("Error code: {}", e);
                e.to_string()
            }
        };
        let cofg: Config = match toml::from_str(&toml_file) {
            Ok(cofg) => cofg,
            Err(_) => {
                eprintln!("{} {}", ">>>".red().bold(), "The log format is old.".bold());
                eprintln!("The current rade log format is toml, but the log format on this computer is the past rade log format");
                eprintln!("Run rm -rf ~/.comrade and reinstall rade");
                eprintln!("{} {}","Note:".bold(),"This action will delete not only rade but all programs previously installed with rade.".red());
                eprintln!("After accepting this, execute the previous command.");
                std::process::exit(1);
            }
        };

        (
            cofg.install.install_name,
            cofg.info.version,
            cofg.info.repositry,
        )
    }
}
