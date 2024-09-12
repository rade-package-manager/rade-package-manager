use chrono::{Datelike, Utc};
use colored::Colorize;
use dirs::home_dir;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub struct Name<'a> {
    pub basedir: &'a Path,
}

impl<'a> Name<'a> {
    /// set path.
    pub fn new(base: &'a Path) -> Self {
        Name { basedir: base }
    }

    /// create package log files.
    pub fn create(
        &self,
        package: &str,
        _install: &str,
        url: String,
        version: String,
    ) -> Result<(), std::io::Error> {
        let _name = self.basedir.join(package);
        println!(
            "{} {}",
            "==>".bold().blue(),
            _name.display().to_string().as_str().bold()
        );
        let mut fis = fs::File::create(&_name)?;
        let utc = Utc::now();
        let data = format!("{}-{}-{}\n", utc.year(), utc.month(), utc.day());
        let time = utc.time().format("%H:%M:%S").to_string();
        fis.write_all("[time]\n".as_bytes())?;
        fis.write_all(data.as_bytes())?;
        fis.write_all(time.as_bytes())?;
        fis.write_all("\n\n[install]\n".as_bytes())?;
        fis.write_all(_install.as_bytes())?;
        fis.write_all("\n\n[version]\n".as_bytes())?;
        fis.write_all(version.as_bytes())?;
        fis.write_all("\n\n[repositry]\n".as_bytes())?;
        fis.write_all(url.as_bytes())?;
        Ok(())
    }
    pub fn remove_program(&self, pkgname: &String) {
        let _name = self.basedir.join(pkgname);
        fs::remove_file(_name).expect("Failed to remove log");
    }
}

pub fn new() {
    let ps = home_dir()
        .expect("Failed to get home")
        .join(".comrade/log/status");
    if ps.exists() {
        println!("Would you like to refresh the log?");
        println!("This action will erase all previous logs");
        print!("[y/n] ");
        io::stdout().flush().unwrap();
        let mut _str = String::new();
        io::stdin().read_line(&mut _str).unwrap();
        let _str = _str.trim();
        if  ["y", "yes", ""].contains(&_str) {
            println!("{} Start creating log...", ">>>".blue().bold());
            println!("{} Deleting status file...", ">>>".green().bold());
            fs::remove_file(
                home_dir()
                    .expect("Failed to get home dir.")
                    .join(".comrade/log/status"),
            )
            .expect("Failed to remove file");
            println!("{} Start creating log...", ">>>".blue().bold());
            let mut fl = fs::File::create(&ps).expect("Failed to create status file.");
            let utc = chrono::Utc::now();
            let data = format!("utc: {}-{}-{}\n", utc.year(), utc.month(), utc.day());
            let time = utc.time().format("%H:%M:%S").to_string();
            let _install = "[install]\n\n";
            let _status = "[status]\n";
            let installed = "\ninstall: 0\n";
            let time = format!("time: {}", time);
            print!("{}", data.as_str().bold());
            io::stdout().flush().unwrap();
            println!("{}", time.as_str().bold());
            println!("{} Create log status...", ">>>".green().bold());
            fl.write_all(_install.as_bytes())
                .expect("Failed to write log");
            fl.write_all(_status.as_bytes())
                .expect("Failed to write log");
            fl.write_all(data.as_bytes()).expect("Failed to write log");
            fl.write_all(time.as_bytes()).expect("Failed to write log");
            fl.write_all(installed.as_bytes())
                .expect("Failed to write log");
            println!("{}", "Log creation completed successfully".bold().cyan());
        } else {
            println!("Clearing the log has been canceled");
            std::process::exit(1);
        }
    } else {
        println!("{} Start creating log...", ">>>".blue().bold());
        let mut fl = fs::File::create(&ps).expect("Failed to create status file.");
        let utc = chrono::Utc::now();
        let data = format!("utc: {}-{}-{}\n", utc.year(), utc.month(), utc.day());
        let time = utc.time().format("%H:%M:%S").to_string();
        let _status = "[status]\n";
        let installed = "\ninstall: 0\n";
        let time = format!("time: {}", time);
        print!("{}", data.as_str().bold());
        io::stdout().flush().unwrap();
        println!("{}", time.as_str().bold());
        println!("{} Create log status...", ">>>".green().bold());
        fl.write_all(_status.as_bytes())
            .expect("Failed to write log");
        fl.write_all(data.as_bytes()).expect("Failed to write log");
        fl.write_all(time.as_bytes()).expect("Failed to write log");
        fl.write_all(installed.as_bytes())
            .expect("Failed to write log");
        println!("{}", "Log creation completed successfully".bold().cyan());
    }
}

pub fn status() {}
