# comrade

comrade is a simple, fast, and safe package manager written in Rust. It builds programs from source using `install.sh`, ensuring that the software is tailored to your environment. <br>
comrade is designed to work in any environment and offers a straightforward installation process without requiring `sudo` privileges.
<br>
## Features

- **Custom Builds**: Every program is built from source using `install.sh`, ensuring that it is optimized for your specific environment.
- **No Sudo Required**: Install packages easily with `rade install <program>`—no need for `sudo` privileges.
- **Easy Updates**: Keep comrade and your installed programs up to date with `rade update; rade upgrade`.
- **Fast and Safe**: Written in Rust, comrade provides a high-speed and secure package management experience.
- **Universal Compatibility**: comrade is designed to work in any environment.

## Installation

To install comrade, run the following command:
## for Linux/Macos
<br>

```bash
curl -sSfL https://github.com/rade-package-manager/rade-installer/releases/download/0.1/installer.sh -o install.sh; chmod +x install.sh; ./install.sh 
```
This command installs the install.sh file and runs install.sh.
After installation, the install.sh file will be automatically deleted.
For more information, please visit [installer repositry](https://github.com/rade-package-manager/rade-installer/)

## For Windows
Download the installer for Windows and run it(x86_84)<br>
[Download](https://github.com/rade-package-manager/rade-installer/releases/download/0.1/ComradePackageManagerInstaller.exe)
After installation, pass the patch to $HOME\.comrade\bin.


## Usage
- **Install a package**:
  ```bash
  rade install <program>
  ```

- **Update comrade**: 
  ```bash
  rade update; rade upgrade
  ```
  

## Contributing
Regarding contributions, the rules have not yet been determined.  
<br>
Please wait a little longer.

## License
<br>
This project is licensed under the MIT License—see the LICENSE file for details.

## Support
Discord:https://discord.com/invite/QUhr9wSxWr
