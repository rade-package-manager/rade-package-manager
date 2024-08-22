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

        let mut depen: String = match File::open(dependencies) {
            Ok(mut o) => {
                let mut dependen = String::new();
                o.read_to_string(&mut dependen)
                    .expect("failed to read file");
                dependen
            }
            Err(r) => {
                eprintln!("{}{}", "Error".red(), ": Failed to get dependencies.");
                eprintln!("Please report this issue to the knife repository.");
                eprintln!("Error code: {}", r);
                std::process::exit(1);
            }
        };

        // get language
        let lang: String = match File::open(language.trim()) {
            Ok(mut o) => {
                let mut lag = String::new();
                o.read_to_string(&mut lag)
                    .expect("failed to read language.");
                lag
            }
            Err(e) => {
                eprintln!("{}{}", "Error".red(), ": Failed to get language.");
                eprintln!("Please report this issue to the knife repository.");
                eprintln!("Error code: {}", e);
                std::process::exit(1);
            }
        };
        // get repository
        let github: String = match File::open(repository) {
            Ok(mut r) => {
                let mut repo = String::new();
                r.read_to_string(&mut repo)
                    .expect("Failed to read repository");
                repo
            }
            Err(e) => {
                eprintln!("{}{}", "Error".red(), ": Failed to get repository.");
                eprintln!("Please report this issue to the knife repository.");
                eprintln!("Error code: {}", e);
                std::process::exit(1);
            }
        };
        let capa: String = match File::open(capacity) {
            Ok(mut f) => {
                let mut capa = String::new();
                f.read_to_string(&mut capa)
                    .expect("Failed to read capacity");
                capa
            }
            Err(e) => {
                eprintln!("{}{}", "Error".red(), ": Failed to get capacity.");
                eprintln!("Please report this issue to the knife repository.");
                eprintln!("Error code: {}", e);
                std::process::exit(1);
            }
        };
        let ver: String = match File::open(version) {
            Ok(mut f) => {
                let mut vers = String::new();
                f.read_to_string(&mut vers).expect("Failed to read version");
                vers
            }
            Err(e) => {
                eprintln!("{}{}", "Error".red(), ": Failed to get version.");
                eprintln!("Please report this issue to the knife repository.");
                eprintln!("Error code: {}", e);
                std::process::exit(1);
            }
        };

        if depen.is_empty() {
            depen = "None".to_string();
        }
        let capa = capa.trim();
        let ver = ver.trim();
        let depen = depen.trim();
        let github = github.trim();

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
