use dirs::home_dir;
use std::path::PathBuf;

use crate::Package;

/// comrade version.
pub const VERSION: &str = "0.8.0";

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
        let home = home_dir()
            .expect("Failed to get home dir")
            .join(".comrade/");
        home
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
        let packagelist = home_dir()
            .expect("Failed to get packagelist")
            .join(".comrade/packagelist/");
        packagelist
    }
}
