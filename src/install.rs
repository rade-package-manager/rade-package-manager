// SPDX-License-Identifier: MIT
// Copyright (c) 2024 17do
// This software is licensed under the MIT License.

use crate::{install, search};
use colored::*;
use dirs::home_dir;
use git2::Repository;
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
    process::{self},
};

pub fn install(program: &String) {
    let search_ = search::search_program(&program);
    let knife_home = home_dir()
        .expect("Failed to get ~/.comrade/")
        .join(".comrade/");
    if search_ {
        let basepath = format!("{}{}", knife_home.join("packagelist/").display(), &program);
        let dependencies = format!("{}{}", basepath, "/dependencies");
        let language = format!("{}{}", basepath, "/language");
        let repository = format!("{}{}", basepath, "/repository");
        let capacity = format!("{}{}", basepath, "/capacity");
        let version = format!("{}{}", basepath, "/version");

        fn open_and_read_file<P: AsRef<Path>>(path: P, read: &str, get: &str) -> String {
            match File::open(path) {
                Ok(mut f) => {
                    let mut result = String::new();
                    f.read_to_string(&mut result)
                        .unwrap_or_else(|_| panic!("failed to read {}", read));
                    result
                }
                Err(e) => {
                    eprintln!("{} Failed to get {}", ">>>".red().bold(), get);
                    eprintln!("Please report this issue to the comrade repository.");
                    eprintln!("Error code: {}", e);
                    std::process::exit(1);
                }
            }
        }

        let depen: String = open_and_read_file(dependencies, "file", "dependencies");

        // get language
        let lang: String = open_and_read_file(language.trim(), "language", "language");

        // get repository
        let github: String = open_and_read_file(repository, "repository", "repository");

        let capa: String = open_and_read_file(capacity, "capacity", "capacity");

        let ver: String = open_and_read_file(version, "version", "version");
        let lang = lang.trim();
        let capa = capa.trim();
        let ver = ver.trim();
        let depen = depen.trim();
        let github = github.trim();
        println!("{} {}", ">>>".green().bold(), "Clone package...".bold());
        if let Err(_) = Repository::clone(&github, knife_home.join("build")) {
            eprintln!("\n{}: Failed to Clone Repository.", "Error".red());
            eprintln!("Please report this issue to the comrade repository");
            std::process::exit(1);
        }
        let exe =
            install::get_program_name(knife_home.join("build/").display().to_string(), program);
        let exeit = knife_home.join("bin/").join(&exe);
        if exeit.exists() {
            println!(
                "{} {}",
                ">>>".red().bold(),
                "The program is already installed!".bold()
            );
            println!(
                "For more information about this program, please visit {}",
                github
            );
            fs::remove_dir_all(knife_home.join("build/")).expect("Failed to remove dir");
            std::process::exit(1);
        }
        let depen = if depen.is_empty() { "None" } else { depen };
        println!("install package: {}", program);
        println!("executable file name: {}", exe);
        println!("capacity: {}bytes", capa);
        println!("language: {}", lang);
        println!("versions: {}", ver);
        println!("dependencies: {}", depen);
        println!("repository: {}", github);
        println!("\ninstall {}?", program);
        print!("[y/n] ");
        io::stdout().flush().unwrap();
        let mut ok_ = String::new();
        io::stdin().read_line(&mut ok_).unwrap();
        let ok_: &str = ok_.trim();
        if ["y", "yes", ""].contains(&ok_) {
            // start Installation
            println!("{} {}", ">>>".green().bold(), "Start Installation".bold());
            print!("{} {}", ">>>".green().bold(), "");
            println!("building...");

            let status_installsh = process::Command::new("sh")
                .arg(knife_home.join("build/install.sh"))
                .current_dir(knife_home.join("build"))
                .status()
                .expect("Failed to start install.sh");
            if !status_installsh.success() {
                println!(
                    "\n{} install.sh failed. Please report this problem to the comrade repository",
                    ">>>".red().bold()
                );
            }
            fs::rename(
                knife_home.join("build/").join(&exe),
                knife_home.join("bin/").join(&exe),
            )
            .expect("Failed to move file");
            fs::remove_dir_all(knife_home.join("build/")).expect("Failed to remove dir");
            println!("{} {}", ">>>".green().bold(), "All done!".bold());
            println!("Installation is complete");
            println!(
                "For more information on {}, please see {}.",
                program, github
            );
            return;
        }
    }
}

pub fn get_program_name(build_dir: String, program: &String) -> String {
    // build_dir
    let exe_name = Path::new(&build_dir).join(".comrade/exe_name");
    if !exe_name.exists() {
        return program.to_string();
    }
    // 一時的に入れるだけ
    let mut str = String::new();
    // filはファイル
    //
    if let Ok(mut fl) = fs::File::open(&exe_name) {
        fl.read_to_string(&mut str).expect("failed to read file");
        return str.trim().to_string();
    } else {
        eprintln!("failed to read file: {}", exe_name.display());
        eprintln!("Please report this issue to the comrade repository");
        std::process::exit(1);
    }
}
