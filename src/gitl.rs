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
    let url = "https://github.com/knife-package-manager/knife-package-list";
    let home = match home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Failed to obtain home directory.\nPlease report this issue to the Knife repository along with the operating system used.
Error");
            std::process::exit(1);
        }
    };
    let path = home.join(".knife/packagelist");
    // remove the packagelist
    println!("updateing package list..");
    println!("removing {}..", path.display());
    if let Err(er) = fs::remove_dir_all(&path) {
        eprintln!("{}{}", "Could not delete directory.\n Please report this issue to the Knife repository\n Error code: ".red(),er);
        std::process::exit(1);
    }

    // clone the packagelist
    println!("cloning {}", url);
    if let Err(error) = Repository::clone(url, &path) {
        eprintln!("{}{}","Failed to retrieve package list.\nPlease submit this issue to the Knife repository.\nError code:".red(),error);
        std::process::exit(1);
    }
    println!("{}", "Successfully updated package list!");
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
        println!("Want to upgrade your Knife?");
        print!("[y/n] ");
        io::stdout().flush().unwrap();
        let mut Sstr = String::new();
        io::stdin().read_line(&mut Sstr).unwrap();
        let Sstr: &str = Sstr.trim();
        if Sstr == "y" || Sstr == "yes" || Sstr == "" {
            let url: &str = "https://github.com/knife-package-manager/knife-package-manager";
            let home = match home_dir() {
                Some(path) => path,
                None => {
                    eprintln!("Failed to obtain home directory. \nPlease report this issue to the Knife repository along with the operating system used.");
                    std::process::exit(1);
                }
            };
            println!("upgrading Knife");
            let path = home.join(".knife/build");
            if path.exists() {
                print!("removing {}...", path.display());
                io::stdout().flush().unwrap();
                if let Err(error_) = fs::remove_dir_all(&path) {
                    eprintln!("{}{}","Error: Failed to remove knife\nPlease report this problem to the knife repository\nError code:".red(),error_);
                    std::process::exit(1);
                }
                println!("ok");
            }

            print!("creating .knife/build...");
            io::stdout().flush().unwrap();
            if let Err(er) = Repository::clone(
                "https://github.com/knife-package-manager/knife-package-manager",
                path,
            ) {
                eprintln!("{}: Failed to get repository", "Error".red());
                eprintln!("Please report this issue to the knife repository");
                eprintln!("Error code: {}", er);
                std::process::exit(1);
            }
            println!("ok");
            print!("removing packagelist...");
            io::stdout().flush().unwrap();
            fs::remove_dir_all(
                home_dir()
                    .expect("failed to get home")
                    .join(".knife/packagelist"),
            )
            .expect("Failed to remove directory");

            print!("creating .knife/packagelist...");
            io::stdout().flush().unwrap();
            // clone package list
            if let Err(error) = Repository::clone(
                "https://github.com/knife-package-manager/knife-package-list",
                home.join(".knife/packagelist"),
            ) {
                eprintln!("{}{}","Failed to retrieve package list.\nPlease submit this issue to the Knife repository.\nError code:".red(),error);

                std::process::exit(1);
            }
            fs::remove_dir_all(
                home_dir()
                    .expect("Failed to get home")
                    .join(".knife/pakcagelist/.git"),
            );
            println!("ok");
            println!("makeing knife...");
            let status = std::process::Command::new("make")
                .current_dir(home.join(".knife/build"))
                .status()
                .expect("failed to execute make.");
            if status.success() {
                println!("ok");
            } else {
                eprintln!(
                    "{}",
                    "Error: Make failed. Please report this issue to the Knife repository"
                );
                std::process::exit(1);
            }
            println!("{}", "All done!".green().bold());
            println!("{}","Knife has been successfully upgraded. Please see the Knife repository for details on the update.".yellow());
            std::process::exit(0);
        } else {
            println!("Upgrade canceled.");
            std::process::exit(0);
        }
    } else {
        println!("knfie is already up-to-date!");
        std::process::exit(0);
    }
}
