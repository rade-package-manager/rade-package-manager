use crate::logparser;
use colored::*;
use std::fs;
use std::io;

fn remove(package: String) {
    let exists = logparser::program_exists(package);
    if exists {}
}
