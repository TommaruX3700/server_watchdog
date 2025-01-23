@echo off
:: Requirements for building on windows machine and pushing to proper repos:
::  > Docker
::  > Rust
::  > Git

:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
:: 1 - build for windows
cargo -vV >nul 2>nul
if %errorlevel% neq 0 (
    echo Rust is not installed or properly configured!  Shutting down . . .
    exit /b 1
)

cargo build --release .\server_watchdog
cargo build --release .\installer
cargo build --release .\unistaller

:: Define the path to the text file, check it and get address
setlocal enabledelayedexpansion
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

:: Trying to push binaries
git remote | findstr /R /C:"^origin$" >nul
if %ERRORLEVEL% neq 0 (
    echo No remote repository found. Trying to add it . . .
    git remote add origin %repo% >nul
    if %errorlevel% neq 0 (
        echo Can't add remote repository! Shutting down . . .
        exit /b 1
    )
) 
git fetch origin

git branch --list win_release >nul
if %ERRORLEVEL% neq 0 (
    git checkout -b win_release origin/win_release
) else (
    git checkout win_release
)

git add .
git commit -m "New windows release"
git push --set-upstream origin win_release >nul 
if %errorlevel% neq 0 (
    echo Not being able to push to remote branch!
)

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

