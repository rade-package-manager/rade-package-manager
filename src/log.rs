#![allow(warnings)]

use chrono::format::format;
use chrono::{Datelike, Utc};
use dirs::home_dir;
use std::fs;
use std::io::Write;
use std::path::Path;

pub struct Name<'a> {
    pub basedir: &'a Path,
}

impl<'a> Name<'a> {
    /// set path.
    pub fn set(base: &'a Path) -> Self {
        Name { basedir: base }
    }
    /// create package log files.
    /// context template:
    /// ```rust
    /// Name::set("$HOME/.comrade/log").create("hello_knife".to_string(),"hello\n");
    /// ```
    pub fn create(&self, package: String, context: &str) -> () {
        let _name = self.basedir.join(package);
        println!("{}", _name.display());
        let mut fis = fs::File::create(_name).expect("Failed to create file");
        fis.write_all(context.as_bytes());
    }
}

pub fn new() {
    let ps = home_dir()
        .expect("Failed to get home")
        .join(".comrade/log/status");
    if !ps.exists() {
        let mut fl = fs::File::create(&ps).expect("Failed to create status file.");
        let utc = chrono::Utc::now();
        let year = utc.year();
        let month = utc.month();
        let day = utc.day();
        let time = utc.time();
        let data = format!("{}-{}-{}", year, month, day);
        let fo = format!("{}", time);
        let fo: Vec<_> = fo.split(".").collect();
        fl.write_all(data.as_bytes()).expect("Failed to write log");
        fl.write_all(fo[0].as_bytes()).expect("Failed to write log");
    }
    println!("{}", ps.display());
    //fs::create_dir("");
}

pub fn status() {}
