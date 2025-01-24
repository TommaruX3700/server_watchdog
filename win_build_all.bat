@echo off
for /F "tokens=2 delims==" %%I in ('"wmic os get localdatetime /value | findstr ="') do set datetime=%%I
set DATE_=%datetime:~6,2%_%datetime:~4,2%_%datetime:~0,4%

mkdir releases\win\win_%DATE_%

cd installer
cargo build --release
move /Y target\* ..\releases\deb_%DATE_%\
cd ..

cd server_watchdog
cargo build --release
move /Y target\* ..\releases\deb_%DATE_%\
cd ..

cd uninstaller
cargo build --release
move /Y target\* ..\releases\deb_%DATE_%\
cd ..

@echo Build and file organization completed.
