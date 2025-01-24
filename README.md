# Server_watchdog
## Description:
Watchdog for linux debian servers.
Monitors resources, processes, and so on.
## How to use:
- use provided executable to install daemon:
    - if not specified otherwise, installation folder will be at `$/server_watchdog/`;
    - installation will try to install all pre-required packages, if they are not installed yet;
    - installation log will be provided in the same executable folder.

- The daemon provides mqtt channels after install:
    - `@server_statistics`
    - `@server_messages`
    - `@server_warnings_verbose`
    - `@server_errors`

- Log files will be provided in installation folder, `tail` it to monitor logs of the program properly;

- Type `server_wtdg --help` for other options.

## Requirements:
- lm-sensors

---
## v0.3 - Pre-Alpha - 23/01/2025
- Dockerfile configuration modified to cross-compile for debian systems and push release-ready binaries to proper release branches
- Developed MQTT Client messages
- Added Installer/Uninstaller
- Added Ping-My-Devices feature
- 

## v0.2 - Pre-Alpha - 07/01/2025
### Updates
- Added Dockerfile to assembly testing container enviroment
- Developing MQTT Client messaging 
- Developing auto-installer

## v0.1 - Pre-Alpha - 10/12/2024
### State:
Project created.

