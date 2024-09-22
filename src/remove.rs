// SPDX-License-Identifier: MIT
// Copyright (c) 2024 17do
// This software is licensed under the MIT License.

use crate::{log, logparser, Package};
use colored::*;
use dirs::home_dir;
use std::{
    fs,
    io::{self, Write},
};

impl Package {
    /// # remove
    /// this function is remove the package.
    /// If you want to use this function in the source code, you can eliminate the selection ([y/n]) when deleting by entering true in the source factor.
    ///
    pub fn remove(package: &str, source: bool) {
        println!(
            "{} {}",
            ">>>".green().bold(),
            "Searching for package...".bold()
        );
        let exists = logparser::program_exists(package);
        if exists {
            let base = home_dir()
                .expect("Failed to get home dir")
                .join(".comrade/");

            let (name, version, repo) = Package::log_parse(package);
            let bytes = format!("{}{}", base.join("bin/").display(), &name);
            let bytes = fs::metadata(bytes).expect("Failed to get metadata").len();
            let mut _str = String::new();
            if !source {
                println!("\n{}{}", "remove package: ".bold(), &package);
                println!("{}{}bytes", "Capacity released: ".bold(), bytes);
                println!(
                    "{}{}",
                    "Executable file name: ".bold(),
                    &name.as_str().bold()
                );
                println!("{}{}", "version: ".bold(), version);
                println!("Repository: {}\n", repo.as_str().bold());
                println!("Do you really want to delete {}?", &package);
                print!("[y/n] ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut _str).unwrap();
                let _str = _str.trim();
                if ["y", "yes", ""].contains(&_str) {
                    println!("{} remove executable file...", ">>>".green().bold());
                    if let Err(r) =
                        fs::remove_file(format!("{}{}", base.join("bin/").display(), &name))
                    {
                        eprintln!("{} Failed to remove executable file", ">>>".red().bold());
                        eprintln!("Please report this issue to the rade repository");
                        eprintln!("Error code: {}", r);
                        std::process::exit(1);
                    }
                    println!("{} remove log file...", ">>>".green().bold());
                    log::Name::new(&base.join("log/install/")).remove_program(package);
                }
            } else {
                println!("{} remove executable file...", ">>>".green().bold());
                if let Err(r) = fs::remove_file(format!("{}{}", base.join("bin/").display(), &name))
                {
                    eprintln!("{} Failed to remove executable file", ">>>".red().bold());
                    eprintln!("Please report this issue to the rade repository");
                    eprintln!("Error code: {}", r);
                    std::process::exit(1);
                }
                println!("{} remove log file...", ">>>".green().bold());
                log::Name::new(&base.join("log/install/")).remove_program(package);
            }
        }
    }
}
