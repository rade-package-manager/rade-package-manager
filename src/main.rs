#![allow(warnings)]
use colored::*;
use git2::Repository;
use std::env;
use std::fs;
use std::io;

struct Gitl {
    clone: bool,
    get_list: String,
}

impl Gitl {
    pub fn load_list() {
        if let Err(_) = fs::read_dir("~/.knife/packagelist/") {
            eprintln!(
                "{}{}{}{}{}",
                "--Error--\n".red().bold(),
                "Failed to retrieve package list.\n".red().bold(),
                "please run \n".red().bold(),
                "knife update\n".cyan(),
                "to retrieve package list".red().bold()
            )
        }
    }
    pub fn clone_from_repo() {}
}

fn main() {
    Gitl::load_list();
}
