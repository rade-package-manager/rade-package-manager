#![allow(warnings)]
use crate::log;
use crate::logparser;
use colored::*;
use core::panic;
use dirs::home_dir;
use std::fs;
use std::io;
use std::io::Write;

pub fn remove(package: String) {
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
        let (name, repo) = logparser::get_exec_name(&package);
        let bytes = format!("{}{}", base.join("bin/").display(), &name);
        let bytes = fs::metadata(bytes).expect("Failed to get metadata").len();
        println!("{}{}", "remove package: ".bold(), &package);
        println!("{}{}{}", "Capacity released: ".bold(), bytes, "bytes");
        println!(
            "{}{}",
            "Executable file name: ".bold(),
            &name.as_str().bold()
        );
        println!("{}{}\n", "Repositry: ", repo.as_str().bold());
        println!("Do you really want to delete {}?", &package);
        print!("[y/n] ");
        io::stdout().flush().unwrap();
        let mut _str = String::new();
        io::stdin().read_line(&mut _str).unwrap();
        let _str = _str.trim();
        if ["y", "yes", ""].contains(&_str) {
            println!("{} {}", ">>>".green().bold(), "remove executable file...");
            if let Err(r) = fs::remove_file(format!("{}{}", base.join("bin/").display(), &name)) {
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
    }
}
