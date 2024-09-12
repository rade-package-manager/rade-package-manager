// SPDX-License-Identifier: MIT
// Copyright (c) 2024 17do
// This software is licensed under the MIT License.

use colored::*;
use std::fs;

pub fn list(dir_path: &str, is_show_file: bool) {
    let dir_path = dirs::home_dir()
        .expect("Failed to get home directory")
        .join(format!(".comrade/{}", dir_path));

    let dir = match fs::read_dir(&dir_path) {
        Ok(dir) => dir,
        Err(_) => {
            eprintln!(
                "{}{}{}{}{}",
                "--Error--\n".red().bold(),
                "Failed to retrieve package list.\n".red().bold(),
                "please run ".red().bold(),
                "rade update ".cyan(),
                "to retrieve package list".red().bold()
            );
            std::process::exit(1);
        }
    };

    for entry in dir {
        let entry = entry.expect("Failed to get entry");
        if entry.path().is_dir() || is_show_file {
            let path = entry.path().display().to_string();
            let path: Vec<&str> = path.split("/").collect();
            let package = path.last().expect("Failed to get package");
            println!("{}", package);
        }
    }
}

#[cfg(target_os = "windows")]
pub fn list(dir_path: &str, is_show_file: bool) {
    let dir_path = dirs::home_dir()
        .expect("Failed to get home directory")
        .join(format!(".comrade/{}", dir_path));

    let dir = match fs::read_dir(&dir_path) {
        Ok(dir) => dir,
        Err(_) => {
            eprintln!(
                "{}{}{}{}{}",
                "--Error--\n".red().bold(),
                "Failed to retrieve package list.\n".red().bold(),
                "please run ".red().bold(),
                "rade update ".cyan(),
                "to retrieve package list".red().bold()
            );
            std::process::exit(1);
        }
    };

    for entry in dir {
        let entry = entry.expect("Failed to get entry");
        if entry.path().is_dir() || is_show_file {
            let path = entry.path().display().to_string();
            let path: Vec<&str> = path.split("¥").collect();
            let package = path.last().expect("Failed to get package");
            println!("{}", package);
        }
    }
}
