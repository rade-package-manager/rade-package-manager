// SPDX-License-Identifier: MIT
// Copyright (c) 2024 17do
// This software is licensed under the MIT License.

#![allow(warnings)]
use crate::log;
use crate::logparser;
use crate::Package;
use colored::*;
use core::panic;
use dirs::home_dir;
use std::fs;
use std::io;
use std::io::Write;

impl Package {
    /// # remove
    /// this function is remove the package.
    /// If you want to use this function in the source code, you can eliminate the selection ([y/n]) when deleting by entering true in the source factor.
    ///
    pub fn remove(package: &String, source: bool) {
        println!(
            "{} {}",
            ">>>".green().bold(),
            "Searching for package...".bold()
        );
        let exists = logparser::program_exists(&package);
        if exists {
            let base = home_dir()
                .expect("Failed to get home dir")
                .join(".comrade/");

            let (name, repo, version) = Package::log_parse(&package);
            let bytes = format!("{}{}", base.join("bin/").display(), &name);
            let bytes = fs::metadata(bytes).expect("Failed to get metadata").len();
            let mut _str = String::new();
            if !source {
                println!("\n{}{}", "remove package: ".bold(), &package);
                println!("{}{}{}", "Capacity released: ".bold(), bytes, "bytes");
                println!(
                    "{}{}",
                    "Executable file name: ".bold(),
                    &name.as_str().bold()
                );
                println!("{}{}", "version: ".bold(), version);
                println!("{}{}\n", "Repositry: ", repo.as_str().bold());
                println!("Do you really want to delete {}?", &package);
                print!("[y/n] ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut _str).unwrap();
                let _str = _str.trim();
                if ["y", "yes", ""].contains(&_str) {
                    println!("{} {}", ">>>".green().bold(), "remove executable file...");
                    if let Err(r) =
                        fs::remove_file(format!("{}{}", base.join("bin/").display(), &name))
                    {
                        eprintln!(
                            "{} {}",
                            ">>>".red().bold(),
                            "Failed to remove executable file"
                        );
                        eprintln!("Please report this issue to the rade repository");
                        eprintln!("Error code: {}", r);
                        std::process::exit(1);
                    }
                    println!("{} {}", ">>>".green().bold(), "remove log file...");
                    log::Name::new(&base.join("log/install/")).remove_program(&package);
                    return;
                } else {
                    return;
                }
            } else {
                println!("{} {}", ">>>".green().bold(), "remove executable file...");
                if let Err(r) = fs::remove_file(format!("{}{}", base.join("bin/").display(), &name))
                {
                    eprintln!(
                        "{} {}",
                        ">>>".red().bold(),
                        "Failed to remove executable file"
                    );
                    eprintln!("Please report this issue to the rade repository");
                    eprintln!("Error code: {}", r);
                    std::process::exit(1);
                }
                println!("{} {}", ">>>".green().bold(), "remove log file...");
                log::Name::new(&base.join("log/install/")).remove_program(&package);

                return;
            }
        }
    }
}
