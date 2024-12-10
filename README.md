# Server_watchdog

## v0.1 - pre-alpha - 10/12/2024
### State:
Under development.
### Description:
Watchdog for linux debian servers.
Monitors resources, processes, and so on.
### Requisites:
- tokio
- sysinfo
- mqtt-client
- serde
- serde-json
- nix
- ctrlc
- tail
### How to use:
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
