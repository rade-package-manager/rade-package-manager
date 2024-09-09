#![allow(warnings)]
use colored::*;
use dirs::home_dir;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::io::BufRead;

pub fn program_exists(packagename: &String) -> bool {
    let dir_path = dirs::home_dir()
        .expect("Failed to get home directory")
        .join(".comrade/log/install");
    let dir = match fs::read_dir(&dir_path) {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!(
                "{}{}{}{}{}",
                ">>> ".red().bold(),
                "Failed to read log files.\n".bold(),
                "Please report this issue to the comrade package manager repository",
                "Error code: ",
                e
            );
            return false;
        }
    };
    let mut found: bool = false;
    let mut ret: bool = false;
    for entry in dir.flatten() {
        if entry.file_name() == <String as AsRef<OsStr>>::as_ref(&packagename) {
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

pub fn get_exec_name(packagename: &String) -> (String, String) {
    let installdir = home_dir()
        .expect("Failed to get home dir")
        .join(".comrade/log/install/");
    let base = format!("{}{}", installdir.display(), &packagename);
    println!("{} {}", "==>".bold().blue(), &base.as_str().bold());
    let mut _install = false;
    let mut rep = false;
    let mut repo = "Error".to_string();
    let mut pkgname = "Error".to_string();
    for result in io::BufReader::new(fs::File::open(base).expect("Failed to open file")).lines() {
        let l = result.unwrap();
        if l.trim() == "[repositry]" {
            _install = false;
            rep = true;
            continue;
        } else if l.trim() == "[install]" {
            _install = true;
            rep = false;
            continue;
        }
        if _install {
            pkgname = l.clone();
        }

        if rep {
            repo = l.clone();
        }
    }
    if pkgname == "Error" {
        eprintln!(
            "{} {}",
            ">>>".red().bold(),
            "Failed to load executable file name".bold()
        );
        eprintln!("{}", "Please report this issue to the comrade repository");
        std::process::exit(1);
    }
    if repo == "Error" {
        eprintln!(
            "{} {}",
            ">>>".red().bold(),
            "Failed to load repositry url".bold()
        );
        eprintln!("{}", "Please report this issue to the comrade repository");
        std::process::exit(1);
    }
    (pkgname.trim().to_string(), repo)
}
