// SPDX-License-Identifier: MIT
// Copyright (c) 2024 17do
// This software is licensed under the MIT License.

use colored::*;
use dirs::home_dir;
use std::ffi::OsStr;
use std::{
    fs,
    io::{self, BufRead},
};

use crate::Package;

pub fn program_exists(packagename: &str) -> bool {
    let dir_path = dirs::home_dir()
        .expect("Failed to get home directory")
        .join(".comrade/log/install");
    let dir = match fs::read_dir(&dir_path) {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!(
                "{}{}Please report this issue to the comrade package manager repositoryError code: {}",
                ">>> ".red().bold(),
                "Failed to read log files.\n".bold(),
                e
            );
            return false;
        }
    };
    let mut found: bool = false;
    let mut ret: bool = false;
    for entry in dir.flatten() {
        if entry.file_name() == <&str as AsRef<OsStr>>::as_ref(&packagename) {
            found = true;
            ret = true;
            break;
        }
    }

    if !found {
        println!(
            "{} {}{}",
            ">>>".red().bold(),
            "Program not found: ".bold(),
            &packagename.bold()
        );
        ret = false;
    }
    ret
}
impl Package {
    /// # log_parse
    /// log paeser.
    /// ## usage
    /// `Package::log_parse(_package_name_)`
    ///
    /// ## return
    /// this function is return the taple
    /// `(String, String, String)`
    /// return list is
    /// `(executable_name, repositry_url, package_version)`
    pub fn log_parse(packagename: &str) -> (String, String, String) {
        let installdir = home_dir()
            .expect("Failed to get home dir")
            .join(".comrade/log/install/");
        let base = format!("{}{}", installdir.display(), &packagename);
        println!("{} {}", "==>".bold().blue(), &base.as_str().bold());
        let mut _install = false;
        let mut rep = false;
        let mut ver = false;
        let mut repo = "Error".to_string();
        let mut pkgname = "Error".to_string();
        let mut version = "Error".to_string();
        for result in io::BufReader::new(fs::File::open(base).expect("Failed to open file")).lines()
        {
            let l = result.unwrap();
            if l.trim() == "[repositry]" {
                _install = false;
                rep = true;
                ver = false;
                continue;
            } else if l.trim() == "[install]" {
                _install = true;
                rep = false;
                ver = false;
                continue;
            } else if l.trim() == "[version]" {
                _install = false;
                rep = false;
                ver = true;
                continue;
            } else if l.trim().is_empty() {
                continue;
            }
            if _install {
                pkgname = l;
            } else if rep {
                repo = l;
            } else if ver {
                version = l;
            }
        }
        if pkgname == "Error" {
            eprintln!(
                "{} {}",
                ">>>".red().bold(),
                "Failed to load executable file name".bold()
            );
            eprintln!("Please report this issue to the comrade repository");
            std::process::exit(1);
        }
        if repo == "Error" {
            eprintln!(
                "{} {}",
                ">>>".red().bold(),
                "Failed to load repositry url".bold()
            );
            eprintln!("Please report this issue to the comrade repository");
            std::process::exit(1);
        }
        if version == "Error" {
            version = "Unable to output version due to old package".to_string();
        }
        (pkgname.trim().to_string(), repo, version)
    }
}
