use colored::*;
use dirs::home_dir;
use git2::Repository;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

// search package list
pub fn search_program(program: &String) -> bool {
    let dir_path = dirs::home_dir()
        .expect("Failed to get home directory")
        .join(".knife/packagelist");

    let dir = match fs::read_dir(&dir_path) {
        Ok(dir) => dir,
        Err(_) => {
            eprintln!(
                "{}{}{}{}{}",
                "--Error--\n".red().bold(),
                "Failed to retrieve package list.\n".red().bold(),
                "please run ".red().bold(),
                "knife update ".cyan(),
                "to retrieve package list".red().bold()
            );
            return false;
        }
    };

    let mut found: bool = false;
    let mut ret: bool = false;
    for entry in dir {
        if let Ok(entry) = entry {
            if entry.file_name().to_string_lossy() == program.clone() {
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
    }

    if !found {
        println!("Program not found: {}", program);
        ret = false;
    }
    ret
}
