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

#[derive(Subcommand, ValueEnum, Clone)]
enum Logs {
    Status,
    New,
    Search,
}

#[derive(Parser)]
#[command(version = "0.7")]
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
    },
    Log {
        logs: Logs,
    },
    Remove {
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
            remove::remove(package);
        }
    }
}
