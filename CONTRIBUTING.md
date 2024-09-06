
# Contributing to comrade package manager

Thank you for your interest in contributing to comrade! We appreciate all contributions, whether they are code improvements, documentation updates, or bug reports.

## How to Contribute

1. **Fork the repository**: Click the "Fork" button at the top of this page to create a copy of the repository under your account.
2. **Clone your fork**: Clone your forked repository to your local machine.
   ```bash
   git clone https://github.com/rade-package-manager/rade-package-manager.git
   ```
3. **Set up the project**:
   comrade requires an openssl development environment on Linux.
   If you are using Mac or Windows, there is no need to worry.
   (If you're using a Debian base such as Ubuntu, you should just need to install libssl-dev and pkg-config)
4. **Create a new branch**:
   Create a new branch for your changes.
   ```bash
   git checkout -b feature/my-new-feature
   ```
5. **Make your changes**:
   Make your code or documentation improvements.
   
6. **Commit your changes**:
   Commit your changes with a descriptive commit message.
   ```bash
   git commit -m "Add new feature: my-new-feature"
   ```
7. **Push your branch**:
   Push your changes to your forked repository.
   ```bash
   git push origin feature/my-new-feature
   ```
8. **Submit a pull request**:
   Open a pull request from your branch to the `stable` branch of this repository.


# Code Style
We use `rustfmt` to ensure consistent code formatting. Before submitting your pull request, please run `rustfmt` on your code:
```bash
cargo fmt
```
You can also configure your editor to automatically format the code on save.
Naming rules for variables, etc.:<br>
If you are using it temporarily and will not use it often, you can use any name you like. However, make sure that the names of functions and variables used in various places are easy to understand.

# Testing
Be sure to carefully test any new features you add to make sure they don't behave strangely.

# Reporting Issues
If you encounter any bugs or have feature requests, please open an issue on GitHub and provide as much detail as possible.

# Communication
If you have questions, feel free to join our community on [Discord]( https://discord.com/invite/QUhr9wSxWr) or reach out via GitHub Discussions.

# License
By contributing to this project, you agree that your contributions will be licensed under the [MIT License](./LICENSE).
