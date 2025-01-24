DATE_ = $date +%d_%m_%Y
mkdir ./releases/deb_$DATE_

cd ./installer
cargo build --release
mv ./target/*  ../releases/deb_$DATE_
cd ..
cd ./server_watchdog
cargo build --release
mv ./target/*  ../releases/deb_$DATE_
cd ..
cd ./uninstaller/
cargo build --release
mv ./target/*  ../releases/deb_$DATE_
cd ..

