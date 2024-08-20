use colored::*;
use dirs::home_dir;
use git2::Repository;
use reqwest::blocking;
use std::env;
use std::fs;
use std::io;
use std::io::Write;


pub struct Gitl {
    clone: bool,
    get_list: String,
}

impl Gitl {
    // update knife package list
    pub fn update_package_list() {
        let url = "https://github.com/17do/knife-package-list";
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
    pub fn upgrade_knife(knife_version: f64) {
        // Confirmation of the version available for pickup
        let upgrading_version = "https://17do.github.io/knife-installer.github.io/";

        // Receive the latest version
        let response: String = blocking::get(upgrading_version)
            .expect("Failed to send GET request")
            .text()
            .expect("Failed to read response text");

        let response = response.trim();

        // get new version
        let new_version: f64 = response
            .parse()
            .expect("Failed to make latest version an float type");

        if new_version > knife_version {
            println!("{}", "Upgrade is valid!".green().bold());
            println!("{} → {}", knife_version, new_version);
            let url: &str = "https://github.com/17do/knife-package-manager";
            let home = match home_dir() {
                Some(path) => path,
                None => {
                    eprintln!("Failed to obtain home directory. \nPlease report this issue to the Knife repository along with the operating system used.");
                    std::process::exit(1);
                }
            };
            let path = home.join(".knife/");
            println!("upgrading Knife");
            print!("removing {}...", path.display());
            io::stdout().flush().unwrap();
            if let Err(error_) = fs::remove_dir_all(&path) {
                eprintln!("{}{}","Error: Failed to remove knife\nPlease report this problem to the knife repository\nError code:".red(),error_);
                std::process::exit(1);
            }
            println!("ok");
            print!("creating .knife...");
            io::stdout().flush().unwrap();
            if let Err(se) = fs::create_dir(home.join(".knife")) {
                eprintln!("{}{}","Failed to create directory\nPlease report this problem to the Knife repository\nError code:".red(),se);
                eprintln!("{}{}{}","P.S.: You can install the latest version of Knife by installing Knife again.\nRun"," curl -sSf https://17do.github.io/knife-installer.github.io/sh.install.html | sh".green()," to install the latest version of Knife.");
                std::process::exit(1);
            }
            println!("ok");
            print!("creating .knife/build...");
            io::stdout().flush().unwrap();
            // clone knife.
            if let Err(er) = Repository::clone(
                "https://github.com/17do/knife-package-manager",
                home.join(".knife/build"),
            ) {
                eprintln!("{}{}","Error: Failed to get knife\nPlease report this problem to the knife repository\nError code:".red(),er);
                eprintln!("{}{}{}","P.S.: You can install the latest version of Knife by installing Knife again.\nRun"," curl -sSf https://17do.github.io/knife-installer.github.io/sh.install.html | sh".green()," to install the latest version of Knife.");
                fs::remove_dir_all(home.join(".knife")).expect("failed to remove directory");
                std::process::exit(1);
            }
            println!("ok");
            print!("creating .knife/packagelist...");
            io::stdout().flush().unwrap();
            // clone package list
            if let Err(error) = Repository::clone(
                "https://github.com/17do/knife-package-list",
                home.join(".knife/packagelist"),
            ) {
                eprintln!("{}{}","Failed to retrieve package list.\nPlease submit this issue to the Knife repository.\nError code:".red(),error);
                eprintln!("{}{}{}","P.S.: You can install the latest version of Knife by installing Knife again.\nRun"," curl -sSf https://17do.github.io/knife-installer.github.io/sh.install.html | sh".green()," to install the latest version of Knife.");
                println!("removing all...");
                fs::remove_dir_all(home.join(".knife")).expect("failed to remove directory");
                std::process::exit(1);
            }
            println!("ok");
            print!("creating .knife/bin...");
            io::stdout().flush().unwrap();
            // cleate ~/.knife/bin/
            if let Err(ree) = fs::create_dir(home.join(".knife/bin")) {
                eprintln!("{}{}","Failed to create directory\nPlease report this problem to the Knife repository\nError code:".red(),ree);
                eprintln!("{}{}{}","P.S.: You can install the latest version of Knife by installing Knife again.\nRun"," curl -sSf https://17do.github.io/knife-installer.github.io/sh.install.html | sh".green()," to install the latest version of Knife.");
                fs::remove_dir_all(home.join(".knife")).expect("failed to remove directory");
                std::process::exit(1);
            }
            println!("ok");
            println!("makeing {}", home.join(".knife/build").display());
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
            println!("Knife is already up to date!");
            std::process::exit(0);
        }
    }
}
