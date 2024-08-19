mkdir ~/.knife/
git clone https://github.com/17do/knife-package-list ~/.knife/packagelist/
mkdir ~/.knife/bin/
mkdir ~/.knife/build
export PATH=$PATH:~/.knife/bin/
git clone https://github.com/17do/knife-package-manager ~/.knife/build
cd ~/.knife/build
chmod +x ./test
./test &&
  make &&
  make install
