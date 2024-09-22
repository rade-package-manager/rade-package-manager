// SPDX-License-Identifier: MIT
// Copyright (c) 2024 17do
// This software is licensed under the MIT License.

mod download_install;
mod gitl;
mod info;
mod install;
mod list;
mod log;
mod logparser;
mod remove;
mod search;
use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;

#[derive(Subcommand, ValueEnum, Clone)]
enum Logs {
    Status,
    New,
    Search,
}
pub struct Package;

#[derive(Parser)]
#[command(version = "0.8.4")]
/// A simple, fast, and safe package manager
enum Cli {
    /// Update the package list
    Update,
    /// Upgrade the knife tool
    Upgrade,
    /// Lists the packages
    List {
        #[arg(short, long)]
        installed: bool,
    },
    /// Install a package
    Install {
        /// The package name (for install command)
        package: String,
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        build: bool,
    },
    /// Rade log managements
    Log { logs: Logs },
    /// Remove the package
    Remove { package: String },
}

fn main() {
    let version = info::VERSION;
    let args = Cli::parse();
    match args {
        Cli::Update => {
            Package::update_package_list();
            std::process::exit(0);
        }
        Cli::Upgrade => {
            gitl::upgrade_rade(version.to_string());
        }
        Cli::Install { package, build } => {
            if build {
                println!("{} {}", ">>>".yellow().bold(), "Selected build".bold());
                Package::install(&package, false, true);
            } else {
                Package::install(&package, false, false);
            }
        }
        Cli::List { installed } => {
            if installed {
                list::list("bin", true);
            } else {
                list::list("packagelist", false);
            }
        }
        Cli::Log { logs } => match logs {
            Logs::New => {
                log::new();
            }
            Logs::Status => {
                log::status();
            }
            Logs::Search => {
                todo!();
            }
        },
        Cli::Remove { package } => {
            Package::remove(&package, false);
        }
    }
}
