use colored::*;
use dirs::home_dir;
use git2::Repository;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

pub fn list(dir_path: &str, is_show_file: bool) {
    let dir_path = dirs::home_dir()
        .expect("Failed to get home directory")
        .join(format!(".knife/{}", dir_path));

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
            std::process::exit(1);
        }
    };

    let mut found: bool = false;
    let mut ret: bool = false;
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
