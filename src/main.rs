#![allow(warnings)]
use colored::*;
use dirs::home_dir;
use git2::Repository;
use std::env;
use std::fs;
use std::io;
mod gitl;
mod info;
use clap::{Arg, Command};
mod search;

fn main() {
    let version = info::VERSION;

    let matches = Command::new("knife")
        .version("0.2")
        .about("A simple, fast, and safe package manager")
        .subcommand(Command::new("update").about("Update the package list"))
        .subcommand(Command::new("upgrade").about("Upgrade the knife tool"))
        .subcommand(
            Command::new("install").about("Install a package").arg(
                Arg::new("package")
                    .help("The package to install")
                    .required(true)
                    .index(1),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("update", _)) => {
            gitl::update_package_list();
            std::process::exit(0);
        }
        Some(("upgrade", _)) => {
            gitl::upgrade_knife(version);
        }
        Some(("install", sub_matches)) => {
            if let Some(package) = sub_matches.get_one::<String>("package") {
                search::search_program(&package.to_string());
            } else {
                eprintln!("{} Specify the package to install", "Error:".red());
            }
        }
        _ => {
            println!(
                "{}{}",
                "Error: command needed.".red(),
                " Please run with the --help option to check your commands."
            );
            std::process::exit(1);
        }
    }
}
