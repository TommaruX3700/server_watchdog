@echo off
:: Requirements for building on windows machine and pushing to proper repos:
::  > Docker
::  > Rust
::  > Git

:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
:: 1 - build for windows and push to release
cargo -vV >nul 2>nul
if %errorlevel% neq 0 (
    echo Rust is not installed or properly configured!  Shutting down . . .
    exit /b 1
)

cargo build .\server_watchdog
cargo build .\installer
cargo build .\unistaller

setlocal enabledelayedexpansion

:: Define the path to the text file, check it and get address
set "file=release_repo"
set "repo="
set "searchKey=repo"

for /F "tokens=1,* delims==" %%A in (file) do (
    if "%%A"=="%searchKey%" (
        set "repo=%%B"
        echo Found %repo%!
    )
)

if %winRepo% == "" (
    echo Couldn't find any repos! Check release_repo file . . .
)

endlocal



:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
:: 2 - build for linux and push to release (done inside docker file)
docker --version >nul 2>nul
if %errorlevel% neq 0 (
    echo Docker is not installed!  Shutting down . . .
    exit /b 1
)

docker run server_watchdog >nul 2>nul
if %errorlevel% neq 0 (
    echo Docker is installed, but not running!
    echo Trying to start Docker. . .

    :: try to build the image and run it in a new container
    docker build -t server_watchdog . && docker run server_watchdog >nul 2>nul
    if %errorlevel% neq 0 (
        echo Cant build or start a new Docker container! Shutting down . . .
        exit /b 1
    )
)

