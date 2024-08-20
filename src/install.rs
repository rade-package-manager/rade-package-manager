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

pub struct Get {}

impl Get {
    // serch package list
    pub fn search_program(program: String) {
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
                return;
            }
        };

        let mut found = false;
        for entry in dir {
            if let Ok(entry) = entry {
                if entry.file_name().to_string_lossy() == program {
                    found = true;
                    let target = entry.path();
                    if target.is_dir() {
                        match fs::read_dir(&target) {
                            Ok(files) => {
                                for file in files {
                                    if let Ok(file) = file {
                                        self::Get::is_repositry(file.path());
                                    }
                                }
                            }
                            Err(e) => println!("Error reading directory contents: {e}"),
                        }
                    } else {
                        println!("  (Not a directory)");
                    }

                    break;
                }
            }
        }

        if !found {
            println!("Program not found: {}", program);
        }
    }

    fn is_repositry(i: PathBuf) -> String {
        let mut content = String::new();
        if let Ok(mut o) = fs::File::open(&i) {
            o.read_to_string(&mut content)
                .expect("Failed to read file.")
                .to_string()
        } else {
            eprintln!("{}failed to open file:{}", "Error".red(), i.display());
            return "Error".to_string();
        }
    }
}
