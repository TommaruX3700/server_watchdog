DATE_=$(date +%d_%m_%Y)

mkdir ./releases/deb_$DATE_
mkdir ./releases/deb_$DATE_/installer
mkdir ./releases/deb_$DATE_/server_watchdog
mkdir ./releases/deb_$DATE_/uninstaller

cd ./installer
cargo build --release
mv ./target/*  ../releases/deb_$DATE_/installer
cd ..
cd ./server_watchdog
cargo build --release
mv ./target/*  ../releases/deb_$DATE_/server_watchdog
cd ..
cd ./uninstaller
cargo build --release
mv ./target/*  ../releases/deb_$DATE_/uninstaller
cd ..

