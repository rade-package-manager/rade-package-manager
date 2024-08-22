# Knife

Knife is a simple, fast, and safe package manager written in Rust. It builds programs from source using `make`, ensuring that the software is tailored to your environment. <br>
Knife is designed to work in any environment and offers a straightforward installation process without requiring `sudo` privileges.
<br>
## Features

- **Custom Builds**: Every program is built from source using `make`, ensuring that it is optimized for your specific environment.
- **No Sudo Required**: Install packages easily with `knife install <program>`—no need for `sudo` privileges.
- **Easy Updates**: Keep Knife and your installed programs up to date with `knife update; knife upgrade`.
- **Fast and Safe**: Written in Rust, Knife provides a high-speed and secure package management experience.
- **Universal Compatibility**: Knife is designed to work in any environment.

## Installation

To install Knife, run the following command:
<br>
```bash
curl -sSfL https://github.com/knife-package-manager/knife-installer/releases/download/0.1/installer.sh -o install.sh; chmod +x install.sh; ./install.sh 
```
This command installs the install.sh file and runs install.sh.
After installation, the install.sh file will be automatically deleted.
For more information, please visit https://github.com/17do/knife-installer.github.io

## Usage


>[!WARNING]
>Since Knife is still under development, the install command has not been implemented yet.<br>
>Please wait for a while
- **Install a package**:
  ```bash
  knife install <program>
  ```

- **Update Knife**: 
  ```bash
  knife update; knife upgrade
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
