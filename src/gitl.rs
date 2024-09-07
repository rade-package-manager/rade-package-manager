// SPDX-License-Identifier: MIT
// Copyright (c) 2024 17do
// This software is licensed under the MIT License.

#![allow(warnings)]
use colored::*;
use dirs::home_dir;
use git2::Repository;
use reqwest::blocking;
use std::env;
use std::fs;
use std::io;
use std::io::Write;

/// update knife package list
pub fn update_package_list() {
    let url = "https://github.com/rade-package-manager/rade-package-list";
    let home = match home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Failed to obtain home directory.\nPlease report this issue to the Knife repository along with the operating system used.
Error");
            std::process::exit(1);
        }
    };
    let path = home.join(".comrade/packagelist");
    // remove the packagelist
    println!(
        "{} {}",
        ">>>".green().bold(),
        "updateing package list".bold()
    );
    if path.exists() {
        println!(
            "{} {} {}..",
            ">>>".green().bold(),
            "removing".bold(),
            path.display().to_string().as_str().bold()
        );
        if let Err(er) = fs::remove_dir_all(&path) {
            eprintln!("{}{}", "Could not delete directory.\n Please report this issue to the comrade repository\n Error code: ".red(),er);
            std::process::exit(1);
        }
    }
    // clone the packagelist
    println!(
        "{} {} {}",
        ">>>".green().bold(),
        "Cloning ".bold(),
        url.bold()
    );
    if let Err(error) = Repository::clone(url, &path) {
        eprintln!("{} {}{}",">>>".red().bold(),"Failed to retrieve package list.\nPlease submit this issue to the comrade repository.\nError code:".bold(),error);

        std::process::exit(1);
    }
    let ps = path.join(".git");
    fs::remove_dir_all(ps).unwrap();
    println!("{}", "Successfully updated package list!".bold());
}

/// upgrade knife
pub fn upgrade_knife(knife_version: String) {
    // Confirmation of the version available for pickup
    let upgrading_version = "https://17do.github.io/knife-installer.github.io/";

    // Receive the latest version
    let new_version: String = blocking::get(upgrading_version)
        .expect("Failed to send GET request")
        .text()
        .expect("Failed to read response text")
        .trim()
        .to_string();

    if new_version != knife_version {
        println!("{}", "Upgrade is valid!".green().bold());
        println!("{} {} {}", knife_version, "→".green().bold(), new_version);
        println!("Want to upgrade your comrade?");
        print!("[y/n] ");
        io::stdout().flush().unwrap();
        let mut Sstr = String::new();
        io::stdin().read_line(&mut Sstr).unwrap();
        let Sstr: &str = Sstr.trim();
        if Sstr == "y" || Sstr == "yes" || Sstr == "" {
            let url: &str = "https://github.com/rade-package-manager/rade-package-manager";
            let home = match home_dir() {
                Some(path) => path,
                None => {
                    eprintln!("Failed to obtain home directory. \nPlease report this issue to the comrade repository along with the operating system used.");
                    std::process::exit(1);
                }
            };
            println!("{} upgrading Comrade", ">>>".green().bold());
            let path = home.join(".comrade/build");
            if path.exists() {
                print!(
                    "{} {} {}",
                    ">>>".green().bold(),
                    "removing".bold(),
                    path.display()
                );
                io::stdout().flush().unwrap();
                if let Err(error_) = fs::remove_dir_all(&path) {
                    eprintln!("{}{}","Error: Failed to remove comrade\nPlease report this problem to the knife repository\nError code:".red(),error_);
                    std::process::exit(1);
                }
            }

            println!(
                "{} {}",
                ">>>".green().bold(),
                "creating .comrade/build".green().bold()
            );

            if let Err(er) = Repository::clone(
                "https://github.com/rade-package-manager/rade-package-manager",
                path,
            ) {
                eprintln!(
                    "{} {}",
                    ">>>".red().bold(),
                    "Failed to get repository".bold()
                );
                eprintln!("Please report this issue to the knife repository");
                eprintln!("Error code: {}", er);
                std::process::exit(1);
            }
            println!("{} {}", ">>>".green().bold(), "removing packagelist".bold());

            fs::remove_dir_all(
                home_dir()
                    .expect("failed to get home")
                    .join(".comrade/packagelist"),
            )
            .expect("Failed to remove directory");
            println!(
                "{} {}",
                ">>>".green().bold(),
                "creating .comrade/packagelist".bold()
            );
            // clone package list
            if let Err(error) = Repository::clone(
                "https://github.com/rade-package-manager/rade-package-list",
                home.join(".comrade/packagelist"),
            ) {
                eprintln!("{} {}{}",">>>".red().bold(),"Failed to retrieve package list.\nPlease submit this issue to the comrade repository.\nError code:".bold(),error.to_string().as_str().red());

                std::process::exit(1);
            }
            fs::remove_dir_all(
                home_dir()
                    .expect("Failed to get home")
                    .join(".comrade/pakcagelist/.git"),
            );

            println!("{} {}", ">>>".yellow().bold(), "starting build".bold());
            let status = std::process::Command::new("make")
                .current_dir(home.join(".comrade/build"))
                .status()
                .expect("failed to execute make.");
            if status.success() {
                println!("ok");
            } else {
                eprintln!(
                    "{}{}",
                    ">>>".red().bold(),
                    " Make failed. Please report this issue to the comrade repository"
                );
                std::process::exit(1);
            }
            println!("{} {}", ">>>".green().bold(), "All done".bold());
            println!("{}","Comrade has been successfully upgraded. Please see the Knife repository for details on the update.".yellow());
            std::process::exit(0);
        } else {
            println!("{} Upgrade canceled.", ">>>".cyan().bold());
            std::process::exit(0);
        }
    } else {
        println!("{} comrade is already up-to-date!", ">>>".yellow().bold());
        std::process::exit(0);
    }
}
