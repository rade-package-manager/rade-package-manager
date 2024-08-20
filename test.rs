use std::process::{Command, Stdio};

fn check_command_exists(command: &str) -> bool {
    let status = Command::new(command)
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match status {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}

fn check_pkg_config_installed() -> bool {
    check_command_exists("pkg-config")
}

fn check_installd_git() -> bool {
    check_command_exists("git")
}

fn check_installd_rust() -> bool {
    check_command_exists("cargo")
}
fn check_installd_make() -> bool {
    check_command_exists("make")
}

fn check_pkg_config_package(package: &str) -> bool {
    let status = Command::new("pkg-config")
        .arg("--exists")
        .arg(package)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match status {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}

fn main() {
    if check_installd_git() {
        println!("git is installed.");
    } else {
        eprintln!("git is required to install knife.");
        eprintln!("Please install git before installing knife.");
        std::process::exit(1);
    }
    if check_installd_rust() {
        println!("cargo is installed.");
    } else {
        eprintln!("Rust is required to install knife.");
        eprintln!("Install rust and then knife.");
        std::process::exit(1);
    }
    if check_installd_make() {
        println!("make is installed.");
    } else {
        eprintln!("Make is required to install Knife.");
        eprintln!("Please install make before installing knife.");
        std::process::exit(1);
    }

    if check_pkg_config_installed() {
        println!("pkg-config is installed.");

        let package = "openssl";
        if check_pkg_config_package(package) {
            println!("{} development package is installed.", package);
        } else {
            eprintln!("{} development package is not installed.", package);
            eprintln!("openssl-dev is required for knife");
            eprintln!("Please install openssl-dev before installing knife");
            std::process::exit(1);
        }
    } else {
        println!("pkg-config is not installed.");
        std::process::exit(1);
    }
}
