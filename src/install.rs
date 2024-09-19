// SPDX-License-Identifier: MIT
// Copyright (c) 2024 17do
// This software is licensed under the MIT License.

use crate::{
    log, Package, {install, search},
};
use colored::*;
use dirs::home_dir;
use git2::Repository;
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
    process::{self},
};

impl Package {
    /// ## get_package_infos
    ///
    /// load the package infos.
    /// this function is return the dependencies, capacity, version, repository, and language.
    /// this function returnd taple
    /// ```rust
    /// (String, String, String, String, String)
    /// ```
    /// and return list is
    /// ```rust
    /// (language, capacity, version, dependencies, repository)
    /// ```
    pub fn get_package_infos(program: &str) -> (String, String, String, String, String) {
        let knife_home = home_dir()
            .expect("Failed to get ~/.comrade/")
            .join(".comrade/");
        let basepath = knife_home.join("packagelist/").join(program);
        let dependencies = basepath.join("dependencies");
        let language = basepath.join("language");
        let repository = basepath.join("repository");
        let capacity = basepath.join("capacity");
        let version = basepath.join("version");

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
        let lang: String = open_and_read_file(language, "language", "language");

        // get repository
        let github: String = open_and_read_file(&repository, "repository", "repository");

        let capa: String = open_and_read_file(capacity, "capacity", "capacity");

        let ver: String = open_and_read_file(version, "version", "version");
        let lang = lang.trim();
        let capa = capa.trim();
        let ver = ver.trim();
        let depen = depen.trim();
        let github = github.trim();
        (
            lang.to_string(),
            capa.to_string(),
            ver.to_string(),
            depen.to_string(),
            github.to_string(),
        )
    }
    /// ## install
    ///
    /// this function is install the package
    ///
    /// ### Usage
    ///
    /// ```rust
    /// install::Package::install(_install_package_name);
    /// ```
    ///
    pub fn install(program: &str, source: bool, build: bool) {
        let search_ = search::search_program(program);
        let mut download = false;
        if !build {
            download = Package::is_download_package(program).unwrap();
        }
        let knife_home = home_dir()
            .expect("Failed to get ~/.comrade/")
            .join(".comrade/");
        if search_ && !download {
            let (lang, capa, ver, depen, github) = Package::get_package_infos(program);
            if home_dir()
                .expect("failed to get home")
                .join(".comrade/build")
                .exists()
            {
                println!(
                    "{} {}",
                    ">>>".green().bold(),
                    "removing build directory...".bold()
                );
                fs::remove_dir_all(
                    home_dir()
                        .expect("Failed to get home")
                        .join(".comrade/build"),
                )
                .expect("Failed to remove build directory");
            }

            println!("{} {}", ">>>".green().bold(), "Clone package...".bold());
            if Repository::clone(&github, knife_home.join("build")).is_err() {
                eprintln!("\n{}: Failed to Clone Repository.", "Error".red());
                eprintln!("Please report this issue to the comrade repository");
                std::process::exit(1);
            }
            let exe =
                install::get_program_name(knife_home.join("build/").display().to_string(), program);
            let exeit = knife_home.join("bin/").join(&exe);
            if exeit.exists() && !source {
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
            let depen = if depen.is_empty() {
                "None"
            } else {
                depen.as_str()
            };
            println!("{} {}", "install package:".bold(), program);
            println!("{} {}", "executable file name:".bold(), exe);
            println!("{} {}bytes", "capacity:".bold(), capa);
            println!("{} {}", "language:".bold(), lang);
            println!("{} {}", "versions:".bold(), ver);
            println!("{} {}", "dependencies:".bold(), depen);
            println!("{} {}", "repository:".bold(), github);
            let mut tmp = String::new();
            let mut ok_ = "yes";
            if !source {
                println!("\n{} {}?", "install".bold(), program);
                print!("[y/n] ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut tmp).unwrap();
                ok_ = tmp.trim();
            }
            if ["y", "yes", ""].contains(&ok_) {
                // start Installation
                println!("{} {}", ">>>".green().bold(), "Start Installation".bold());
                println!("{} run install.sh (build start)", ">>>".yellow().bold());

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
                    std::process::exit(1);
                }
                println!("{} {}", ">>>".cyan().bold(), "build end".bold());
                println!("{} {}", ">>>".green().bold(), "moving file...".bold());
                fs::rename(
                    knife_home.join("build/").join(&exe),
                    knife_home.join("bin/").join(&exe),
                )
                .expect("Failed to move file");
                println!(
                    "{} {}",
                    ">>>".green().bold(),
                    "remove build directory...".bold()
                );
                fs::remove_dir_all(knife_home.join("build/")).expect("Failed to remove dir");
                println!("{} {}", ">>>".green().bold(), "Fill in the log...".bold());
                let _ = log::Name::new(&knife_home.join("log/install/")).create(
                    program,
                    exe.as_str(),
                    github.to_string(),
                    ver.to_string(),
                );
                if !source {
                    println!("{}", "All done!".bold());
                    println!("Installation is complete");
                    println!(
                        "For more information on {}, please see {}.",
                        program, github
                    );
                }
            }
        } else if download {
            let pkg = program;
            let (lang, capa, ver, depen, github) = Package::get_package_infos(program);
            let exe = Package::download_get_execname(pkg).expect("Failed to get exec_name");
            let exeit = knife_home.join("bin/").join(&exe);
            if exeit.exists() && !source {
                println!(
                    "{} {}",
                    ">>>".red().bold(),
                    "The program is already installed!".bold()
                );
                println!(
                    "For more information about this program, please visit {}",
                    github
                );
                std::process::exit(1);
            }
            println!("{} {}", "install package:".bold(), program);
            println!("{} {}", "executable file name:".bold(), exe);
            println!("{} {}bytes", "capacity:".bold(), capa);
            println!("{} {}", "language:".bold(), lang);
            println!("{} {}", "versions:".bold(), ver);
            println!("{} {}", "dependencies:".bold(), depen);
            println!("{} {}", "repository:".bold(), github);
            let mut tmp = String::new();
            let mut ok_ = "yes";
            if !source {
                println!("\n{} {}?", "install".bold(), program);
                print!("[y/n] ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut tmp).unwrap();
                ok_ = tmp.trim();
            }
            if ["y", "yes", ""].contains(&ok_) {
                let archive = Package::download_install(program);
                let _ = Package::unpack_package(archive.unwrap(), program);
                println!("{} {}", ">>>".green().bold(), "Fill in the log...".bold());
                let _ = log::Name::new(&knife_home.join("log/install")).create(
                    program,
                    Package::download_get_execname(pkg)
                        .expect("Failed to get exec_name")
                        .as_str(),
                    github.to_string(),
                    ver.to_string(),
                );
            } else {
                return;
            }
        }
    }
}

pub fn get_program_name(build_dir: String, program: &str) -> String {
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
