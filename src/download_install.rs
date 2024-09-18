// SPDX-License-Identifier: MIT
// Copyright (c) 2024 17do
// This software is licensed under the MIT License.

#![allow(warnings)]
use crate::{log, Package};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::get;
use reqwest::{blocking, blocking::Client};
use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, copy, BufRead, BufReader, Read, Write};
use std::mem::replace;
use std::path::Path;
use std::result::Result;
use std::thread::sleep;
use std::time::Duration;
use std::{env, fs, process};
use zip::read::ZipArchive;

impl Package {
    pub fn download_install(
        package: &String,
    ) -> Result<ZipArchive<BufReader<File>>, Box<dyn Error>> {
        let temp = Package::rade_packagelist().join("temp");
        let output = temp.to_str().unwrap();
        let (url, downloadfilename) = if std::env::consts::OS == "windows" {
            (format!(
            "https://github.com/rade-package-manager/rade-download-lists/releases/download/{}/{}-x86_64-pc-windows-gnu.radepkg",
            package, package
        ),format!("{}-x86_64-pc-windows-gnu.radepkg",package)
        )
        } else if std::env::consts::OS == "macos" {
            (format!(
            "https://github.com/rade-package-manager/rade-download-lists/releases/download/{}/{}-aarch64-apple-darwin.radepkg",
            package, package
        ),format!("{}-aarch64-apple-darwin.radepkg",package)
        )
        } else if std::env::consts::OS == "linux" {
            (format!(
            "https://github.com/rade-package-manager/rade-download-lists/releases/download/{}/{}-x86_64-unknown-linux-gnu.radepkg",
            package, package),format!("{}-x86_64-unknown-linux-gnu.radepkg",package)
        )
        } else {
            eprintln!("Not support os.");
            std::process::exit(1);
            ("".to_string(), "".to_string())
        };
        println!(
            "{} {} {}",
            ">>>".green().bold(),
            "Downloading".bold(),
            downloadfilename
        );
        let client = Client::new();

        let response = client.head(url.clone()).send()?;
        let total_size = response
            .headers()
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|len| len.to_str().ok())
            .and_then(|len| len.parse().ok())
            .unwrap_or(0);

        let progress_bar = ProgressBar::new(total_size);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("[{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
                .progress_chars("###"),
        );
        let mut downloaded: u64 = 0;
        let mut buffer = vec![0; 8192];
        let mut response = client.get(url.clone()).send()?;
        let mut file = File::create(output.clone())?;
        while let Ok(n) = response.read(&mut buffer) {
            if n == 0 {
                break;
            }
            file.write_all(&buffer[..n])?;
            downloaded += n as u64;
            progress_bar.set_position(downloaded);
        }

        file.flush()?;
        let file = File::open(output)?;
        progress_bar.finish();
        let mut reader = BufReader::new(file);
        let mut archive = ZipArchive::new(reader)?;
        return Ok(archive);
    }

    pub fn unpack_package(
        mut archive: ZipArchive<BufReader<File>>,
        package: &String,
    ) -> Result<(), Box<dyn Error>> {
        let build_dir = Package::rade_home().join("build/");
        if build_dir.exists() {
            fs::remove_dir_all(&build_dir)?;
        }
        fs::create_dir(&build_dir)?;

        let mut exec_name = String::new();
        println!(
            "{} {} {}",
            ">>>".yellow().bold(),
            "Unpacking".bold(),
            package.as_str().bold()
        );
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;

            // ファイルのパスを取り出してからファイルの処理を行う
            let outpath = match file.enclosed_name() {
                Some(path) => path,
                None => continue,
            };

            // パスのディレクトリ部分を作成する
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }

            // コピーするファイルのパスを決定する
            let outfile_path = build_dir.join(outpath);
            let mut outfile = fs::File::create(&outfile_path)?;
            std::io::copy(&mut file, &mut outfile)?;
            let outfile = match file.enclosed_name() {
                Some(path) => path,
                None => continue,
            };

            // ファイル名が "exec_name" であれば、内容を読み込む
            if outfile.file_name() == Some(OsStr::new("exec_name")) {
                let content = fs::read_to_string(&outfile_path)?;
                exec_name = content.trim().to_string();
            }
        }

        if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
            let status = process::Command::new("chmod")
                .arg("+x")
                .arg(&exec_name)
                .current_dir(&build_dir)
                .status()?;
            if !status.success() {
                eprintln!("Failed to chmod.");
                std::process::exit(1);
            }
        }

        let home = Package::rade_home();
        let bin = Package::rade_home().join("bin/");
        println!("{} {}", ">>>".yellow().bold(), "Run install.sh".bold());
        Package::parse_sh(&home.join("build/install.sh").as_path())?;

        if !bin.is_dir() {
            panic!("Error: 'bin' is not a directory");
        }
        if !home.join("build/").join(&exec_name).exists() {
            panic!("Error: exec_name file does not exist at the expected path");
        }
        println!("{} {}", ">>>".green().bold(), "move file...".bold());
        fs::rename(home.join("build/").join(&exec_name), bin.join(&exec_name))?;
        println!(
            "{} {}",
            ">>>".green().bold(),
            "Remove build directory...".bold()
        );
        fs::remove_dir_all(&build_dir)?;
        Ok(())
    }
    fn parse_sh(path: &Path) -> Result<(), Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        for result in reader.lines() {
            let l = result?;
            if l.trim() == "__install_end__" {
                return Ok(());
            } else {
                process::Command::new(l.trim()).spawn()?;
            }
        }
        Ok(())
    }
    pub fn is_download_package(package: &String) -> Result<bool, Box<dyn Error>> {
        let packagelist = Package::rade_packagelist().join(package);
        let dir = match fs::read_dir(&packagelist) {
            Ok(dir) => dir,
            Err(_) => {
                std::process::exit(1);
            }
        };
        for entry in dir {
            let entry = entry.unwrap();
            if entry.file_name().into_string().unwrap() == "download" {
                return Ok(true);
            }
        }
        Ok(false)
    }
    pub fn download_get_execname(mut package: &String) -> Result<String, Box<dyn Error>> {
        let pkg = format!(
            "{}{}/exec_name",
            Package::rade_packagelist().display(),
            package
        );
        let f = fs::read_to_string(pkg)?;
        Ok(f.trim().to_string())
    }
}
