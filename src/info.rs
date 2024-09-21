// SPDX-License-Identifier: MIT
// Copyright (c) 2024 17do
// This software is licensed under the MIT License.

use dirs::home_dir;
use std::path::PathBuf;

use crate::Package;

/// comrade version.
pub const VERSION: &str = "0.8.3";

impl Package {
    /// # rade_home()
    /// return is PathBuf.
    /// $HOME/.comrade/
    ///
    /// return:
    /// ```rust
    /// home_dir().expect("Failed to get home dir").join(".comrade/")
    /// ```
    pub fn rade_home() -> PathBuf {
        home_dir()
            .expect("Failed to get home dir")
            .join(".comrade/")
    }
    /// # rade_packagelist()
    /// return is PathBuf.
    /// $HOME/.comrade/packagelist/
    ///
    /// return:
    /// ```rust
    /// home_dir().expect("Failed to get packagelist").join(".comrade/packagelist/")
    /// ```
    pub fn rade_packagelist() -> PathBuf {
        home_dir()
            .expect("Failed to get packagelist")
            .join(".comrade/packagelist/")
    }
}
