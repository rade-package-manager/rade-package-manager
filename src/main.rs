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
use clap::Parser;

#[derive(Parser)]
/// A simple, fast, and safe package manager
enum Cli {
    Update,
    Upgrade,
    Install {
        /// The package name (for install command)
        package: String
    },
}

fn main() {
    let version = info::VERSION;

    let args = Cli::parse();

    match args {
        Cli::Update => {
            gitl::Gitl::update_package_list();
        }
        Cli::Upgrade => {
            gitl::Gitl::upgrade_knife(version);
        }
        Cli::Install { package } => {
            install::get::search_program(package)
        }
    }
}
