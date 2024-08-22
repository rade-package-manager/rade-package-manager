use crate::search;
use colored::*;
use dirs::home_dir;
use git2::Repository;
use std::env;
use std::fmt::write;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;

use std::process;
use std::process::Stdio;

pub fn install(program: &String) {
    let search_ = search::search_program(&program);
    let knife_home = home_dir().expect("Failed to get ~/.knife/").join(".knife/");
    if search_ {
        let basepath = format!("{}{}", knife_home.join("packagelist/").display(), &program);
        let dependencies = format!("{}{}", basepath, "/dependencies");
        let language = format!("{}{}", basepath, "/language");
        let repository = format!("{}{}", basepath, "/repository");
        let capacity = format!("{}{}", basepath, "/capacity");
        let version = format!("{}{}", basepath, "/version");

        let throw_error = |r| {
            eprintln!("{}{}", "Error".red(), ": Failed to get dependencies.");
            eprintln!("Please report this issue to the knife repository.");
            eprintln!("Error code: {}", r);
            std::process::exit(1);
        };

        let mut depen: String = fs::read_to_string(dependencies).unwrap_or_else(throw_error);

        // get language
        let lang: String = fs::read_to_string(language.trim()).unwrap_or_else(throw_error);

        // get repository
        let github: String = fs::read_to_string(repository).unwrap_or_else(throw_error);

        let capa: String = fs::read_to_string(capacity).unwrap_or_else(throw_error);

        let ver: String = fs::read_to_string(version).unwrap_or_else(throw_error);

        let capa = capa.trim();
        let ver = ver.trim();
        let mut depen = depen.trim();
        let github = github.trim();

        if depen.is_empty() {
            depen = "None";
        }

        println!("install package: {}", program);
        println!("capacity: {}bytes", capa);
        println!("versions: {}", ver);
        println!("dependencies: {}", depen);
        println!("repository: {}", github);
        println!("\ninstall {}?", program);
        print!("[y/n] ");
        io::stdout().flush().unwrap();
        let mut ok_ = String::new();
        io::stdin().read_line(&mut ok_).unwrap();
        let ok_: &str = ok_.trim();

        if (ok_ == "y" || ok_ == "yes" || ok_ == "") {
            // start installation
            fs::remove_dir_all(knife_home.join("build/"));
            print!("cloning Repository...");
            io::stdout().flush().unwrap();

            if let Err(e) = Repository::clone(&github, knife_home.join("build")) {
                eprintln!("\n{}: Failed to Clone Repository.", "Error".red());
                eprintln!("Please report this issue to the knife repository");
                std::process::exit(1);
            }
            println!("ok");
            print!("chmod + ~/.knife/build/install.sh...");
            io::stdout().flush().unwrap();
            if knife_home.join("build/install.sh").exists() {
                let status_chmod = process::Command::new("chmod")
                    .arg("+x")
                    .arg(knife_home.join("build/install.sh"))
                    .current_dir(knife_home.join("build"))
                    .stdout(Stdio::null())
                    .status()
                    .expect("Failed to start chmod");

                if !status_chmod.success() {
                    eprintln!(
                        "{}: chmod failed. Please report this problem to the knife repository",
                        "Error".red()
                    );
                    process::exit(1);
                }
                println!("ok");
                print!("building...");
                io::stdout().flush().unwrap();
                let status_installsh = process::Command::new("sh")
                    .arg(knife_home.join("build/install.sh"))
                    .current_dir(knife_home.join("build"))
                    .status()
                    .expect("Failed to start install.sh");
                if !status_installsh.success() {
                    println!(
                        "install.sh failed. Please report this problem to the KNIFE repository"
                    );
                }

                let name = format!("{}{}", knife_home.join("build/").display(), program);
                fs::rename(name, knife_home.join("bin/").join(program))
                    .expect("Failed to move file");
                return;
            }
        }
    }
}
