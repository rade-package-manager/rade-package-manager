#![allow(warnings)]
use colored::*;
use dirs::home_dir;
use git2::Repository;
use search::search_program;
use std::env;
use std::fs;
use std::io;
mod gitl;
mod info;
mod install;
mod search;
use clap::Parser;

#[derive(Parser)]
#[command(version = "0.3")]
/// A simple, fast, and safe package manager
enum Cli {
    /// Update the package list
    Update,
    /// Upgrade the knife tool
    Upgrade,
    /// Install a package
    Install {
        /// The package name (for install command)
        package: String,
    },
}

fn main() {
    let version = info::VERSION;

    let args = Cli::parse();

    match args {
        Cli::Update => {
            gitl::update_package_list();
            std::process::exit(0);
        }
        Cli::Upgrade => {
            gitl::upgrade_knife(version.to_string());
        }
        Cli::Install { package } => {
            install::install(&package);
        }
    }
}
