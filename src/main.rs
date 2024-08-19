#![allow(warnings)]
use colored::*;
use git2::Repository;
use std::env;
use std::fs;
use std::io;
use std::os::unix::process;

struct Gitl {
    clone: bool,
    get_list: String,
}

impl Gitl {
    pub fn load_list() {
        match fs::read_dir("~/.knife/packagelist") {
            Ok(dir) => {}
            Err(_) => {
                eprintln!(
                    "{}{}{}{}{}",
                    "--Error--\n".red().bold(),
                    "Failed to retrieve package list.\n".red().bold(),
                    "please run ".red().bold(),
                    "knife update ".cyan(),
                    "to retrieve package list".red().bold()
                )
            }
        }
    }
    pub fn update_package_list() {
        let url = "https://github.com/17do/knife-package-list";
        let path = "~/.knife/packagelist";
        // remove the packagelist
        println!("updateing package list..");
        println!("removing {}..", path);
        if let Err(er) = fs::remove_dir_all(path) {
            eprintln!("{}{}", "Could not delete directory.\n Please report this issue to the Knife repository\n Error code: ".red(),er);
            std::process::exit(1);
        }

        // clone the packagelist
        println!("cloning {}", url);
        if let Err(error) = Repository::clone(url, path) {
            eprintln!("{}{}","Failed to retrieve package list.\nPlease submit this issue to the Knife repository.\nError code:".red(),error);
            std::process::exit(1);
        }
        println!("{}", "Successfully updated package list!");
    }
}

fn main() {
    let _args: Vec<String> = env::args().collect();
    if _args.len() > 1 {
        if _args[1] == "update" {
            Gitl::update_package_list();
            std::process::exit(0);
        }
    } else {
        println!(
            "{}{}",
            "Error: option needed.".red(),
            " Please run --help option and check your options"
        );
        std::process::exit(1);
    }
}
