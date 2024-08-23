#!/bin/sh

printf "Knife Installer\n"
printf "This installer builds Knife and generates an executable file tailored to your computer.\n"
printf "Estimated build time is about 4 minutes.\n"
printf "Enter y or yes to run, or n or no to cancel\n"
printf "PS: If you want to install knife from dev branch (develop), type d or dev\n"
printf "[y/n] "
read val

ok=false
current_dir=$(pwd)

if [ "$val" = "y" ] || [ "$val" = "yes" ] || [ "$val" = "" ]; then
  echo "Start installation"
  if [ -d "$HOME/.knife" ] && [ -e "$HOME/.knife/bin/knife" ]; then
  printf "knife is already installed!\n"
  printf "If you still want to install it, you can reinstall knife with knife upgrade!\n"
  printf "Or clone the knife repository to ~/.knife/build and use make to reinstall knife yourself!\n"
   cd "$current_dir" &&
    rm install.sh
    exit 1
  fi
  printf "creating knife_build..."
  mkdir -p "$HOME/knife_build" &&
    printf "ok\n"
  printf "downloading the test.rs..."
  curl -sSfL https://github.com/knife-package-manager/knife-installer.github.io/releases/download/0.1/check_knife_install_dependency.rs -o "$HOME/knife_build/test.rs" &&
    printf "ok\n"
  printf "checking rustc..."
  if which rustc >"$HOME/knife_build/info.log"; then
    printf "ok\n"
  else
    printf "\nrustc is not installed. rustc is required for Knife installation.\nPlease install Rust before installing knife\n."
    exit 1
  fi
  printf "compiling test.rs..."
  rustc "$HOME/knife_build/test.rs" -o "$HOME/knife_build/test" &&
    printf "ok\n" &&
    printf "Checking to see if the program is installed...\n"
  if "$HOME/knife_build/test"; then
    printf ""
  else
    printf "Test program failed to run.\n"
    exit 1
  fi
  printf "creating ~/.knife..."
  mkdir -p ~/.knife &&
    printf "ok\n"
  printf "cloning knife-package-list..."
  printf "===clone packagelist===\n" >>"$HOME/knife_build/info.log"
  git clone --quiet https://github.com/knife-package-manager/knife-package-list ~/.knife/packagelist/ &&
    printf "ok\n"
    printf "clone is ok"
  printf "creating ~/.knife/bin/..."
  mkdir -p ~/.knife/bin &&
    printf "ok\n"
  printf "creating ~/.knife/build/..."
  mkdir -p ~/.knife/build &&
    printf "ok\n"
  printf "cloning knife..."
  printf "===clone knife-package-manager===\n" >>"$HOME/knife_build/info.log"
  git clone --quiet https://github.com/knife-package-manager/knife-package-manager ~/.knife/build &&
    printf "ok\n"
  printf "cd ~/.knife/build/..."
  cd ~/.knife/build &&
    printf "ok\n"
  printf "making...\n"
  if make install; then
    printf "All done!\n"
    printf "Knife installed successfully!\nFor more information about Knife, please visit the Knife Repository\n"
    ok=true
  else
    printf "Build failed.\n"
    exit 1
  fi

  if [ -f ~/.bashrc ] && [ -r ~/.bashrc ]; then
    echo 'export PATH=$PATH:$HOME/.knife/bin' >>~/.bashrc
  fi
  if [ -f ~/.zshrc ] && [ -r ~/.zshrc ]; then
    echo 'export PATH=$PATH:$HOME/.knife/bin' >>~/.zshrc
  fi
  if [ -f ~/.config/fish/config.fish ] && [ -r ~/.config/fish/config.fish ]; then
    echo 'fish_add_path ~/.knife/bin' >>~/.config/fish/config.fish
  fi

  if [ -d ~/.knife/build ]; then
    find ~/.knife/build -mindepth 1 -delete
  fi

elif [ "$val" = "n" ] || [ "$val" = "no" ]; then
  echo "Installation has been canceled."
  cd "$current_dir" &&
    rm install.sh
  

elif [ "$val" = "d" ] || [ "$val" = "dev" ]; then
  echo "Start installation"
  if [ -d "$HOME/.knife" ] && [ -e "$HOME/.knife/bin/knife" ]; then
  printf "knife is already installed!\n"
  printf "If you still want to install it, you can reinstall knife with knife upgrade!\n"
  printf "Or clone the knife repository to ~/.knife/build and use make to reinstall knife yourself!\n"
   cd "$current_dir" &&
    rm install.sh
    exit 1
  fi
  printf "creating knife_build..."
  mkdir -p "$HOME/knife_build" &&
    printf "ok\n"
  printf "downloading the test.rs..."
  curl -sSfL https://github.com/knife-package-manager/knife-installer.github.io/releases/download/0.1/check_knife_install_dependency.rs -o "$HOME/knife_build/test.rs" &&
    printf "ok\n"
  printf "checking rustc..."
  if which rustc >"$HOME/knife_build/info.log"; then
    printf "ok\n"
  else
    printf "\nrustc is not installed. rustc is required for Knife installation.\nPlease install Rust before installing knife\n."
    cd "$current_dir" &&
    rm install.sh
    rm -rf "$HOME/.knife"
    exit 1
  fi
  printf "compiling test.rs..."
  rustc "$HOME/knife_build/test.rs" -o "$HOME/knife_build/test" &&
    printf "ok\n" &&
    printf "Checking to see if the program is installed...\n"
  if "$HOME/knife_build/test"; then
    printf ""
  else
    printf "Test program failed to run.\n"
    exit 1
  fi
  printf "creating ~/.knife..."
  mkdir -p ~/.knife &&
    printf "ok\n"
  printf "cloning knife-package-list..."
  printf "===clone packagelist===\n" >>"$HOME/knife_build/info.log"
  git clone --quiet https://github.com/knife-package-manager/knife-package-list ~/.knife/packagelist/ &&
    printf "ok\n"
    printf "clone is ok\n" >>"$HOME/knife_build/info.log"
  printf "creating ~/.knife/bin/..."
  mkdir -p ~/.knife/bin &&
    printf "ok\n"
  printf "creating ~/.knife/build/..."
  mkdir -p ~/.knife/build &&
    printf "ok\n"
  printf "cloning knife..."
  printf "===clone knife-package-manager (dev)===\n" >>"$HOME/knife_build/info.log"
  git clone --quiet -b dev  https://github.com/knife-package-manager/knife-package-manager ~/.knife/build &&
    printf "ok\n"
  printf "cd ~/.knife/build/..."
  cd ~/.knife/build &&
    printf "ok\n"
  printf "making...\n"
  if make install; then
    printf "All done!\n"
    printf "Knife installed successfully!\nFor more information about Knife, please visit the Knife Repository\n"
    ok=true
  else
    printf "Build failed.\n"
    exit 1
  fi

  if [ -f ~/.bashrc ] && [ -r ~/.bashrc ]; then
    echo 'export PATH=$PATH:$HOME/.knife/bin' >>~/.bashrc
  fi
  if [ -f ~/.zshrc ] && [ -r ~/.zshrc ]; then
    echo 'export PATH=$PATH:$HOME/.knife/bin' >>~/.zshrc
  fi
  if [ -f ~/.config/fish/config.fish ] && [ -r ~/.config/fish/config.fish ]; then
    echo 'fish_add_path ~/.knife/bin' >>~/.config/fish/config.fish
  fi

  if [ -d ~/.knife/build ]; then
    find ~/.knife/build -mindepth 1 -delete
  fi


else
  echo "Invalid input. Please enter y or yes to run, or n or no to cancel."
  cd "$current_dir" &&
    rm install.sh
fi

if [ "$ok" = true ]; then
  cd "$current_dir" &&
    rm -rf "$HOME/knife_build/"
    rm install.sh
fi
