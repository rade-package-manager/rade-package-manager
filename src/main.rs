#![allow(warnings)]
use colored::*;
use dirs::home_dir;
use git2::Repository;
use std::env;
use std::fs;
use std::io;
mod gitl;
mod info;
mod install;
use clap::{Arg, Command};

fn main() {
    let version = info::VERSION;

    let matches = Command::new("knife")
        .about("A simple, fast, and safe package manager")
        .arg(
            Arg::new("command")
                .help("The command to execute")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("package")
                .help("The package name (for install command)")
                .required(false)
                .index(2),
        )
        .get_matches();

    match matches.get_one::<String>("command").unwrap().as_str() {
        "update" => {
            gitl::Gitl::update_package_list();
            std::process::exit(0);
        }
        "upgrade" => {
            gitl::Gitl::upgrade_knife(version);
        }
        "install" => {
            if let Some(package) = matches.get_one::<String>("package") {
                install::get::search_program(package.to_string());
            } else {
                eprintln!("{} Specify the package to install", "Error:".red());
            }
        }
        _ => {
            println!(
                "{}{}",
                "Error: option needed.".red(),
                " Please run with the --help option to check your options."
            );
            std::process::exit(1);
        }
    }
}
