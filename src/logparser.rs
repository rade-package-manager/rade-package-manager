#![allow(warnings)]
use colored::*;
use dirs::home_dir;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::io::BufRead;

pub fn program_exists(packagename: String) -> bool {
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
            let target = entry.path();
            if target.is_dir() {
                ret = true;
            } else {
                found = false;
            }

            break;
        }
    }

    if !found {
        println!("Program not found: {}", &packagename);
        ret = false;
    }
    ret
}

pub fn get_exec_name(packagename: String) {
    let installdir = home_dir()
        .expect("Failed to get home dir")
        .join(".comrade/log/install/");
    let base = format!("{}{}", installdir.display(), &packagename);
    let mut _install = false;
    let mut pkgname = "Error";
    for result in io::BufReader::new(fs::File::open(base).expect("Failed to open file")).lines() {
        let l = result.unwrap();
        let l = l.trim();
        if l == "[install]" {
            _install = true;
        }
        if _install {
            pkgname = l.clone();
        }
    }
}
