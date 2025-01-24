use std::fs;
use std::io::{self, BufRead};
use std::process::Command;
use std::net::{IpAddr, ToSocketAddrs};

/// Reads hostnames from the given file, resolves them to IP addresses, and pings them.
pub fn process_hostnames_from_file(filename: &str) -> Result<(), String> {
    // Read the list of hostnames from the file
    let hostnames = read_hosts_from_file(filename).map_err(|e| e.to_string())?;

    // Resolve each hostname to an IP address and ping them
    for hostname in hostnames {
        match resolve_hostname(&hostname) {
            Ok(ip) => {
                if let Err(e) = ping(ip) {
                    eprintln!("Ping failed for {}: {}", hostname, e);
                }
            }
            Err(e) => eprintln!("Error resolving {}: {}", hostname, e),
        }
    }
    Ok(())
}

/// Reads hostnames from the given file.
fn read_hosts_from_file(filename: &str) -> Result<Vec<String>, io::Error> {
    let file = fs::File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut hosts = Vec::new();
    
    for line in reader.lines() {
        if let Ok(line) = line {
            let trimmed_line = line.trim();
            if !trimmed_line.is_empty() {
                hosts.push(trimmed_line.to_string());
            }
        }
    }
    
    Ok(hosts)
}

/// Resolves a hostname to an IP address.
fn resolve_hostname(hostname: &str) -> Result<IpAddr, String> {
    match hostname.to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(ip) = addrs.next() {
                Ok(ip.ip())
            } else {
                Err(format!("Could not resolve {}", hostname))
            }
        }
        Err(e) => Err(format!("Error resolving {}: {}", hostname, e)),
    }
}

/// Executes the `ping` command for the given IP address.
fn ping(ip: IpAddr) -> Result<(), String> {
    let ip_str = ip.to_string();
    
    // Execute the `ping` command with the `-c 4` flag (ping 4 times)
    let output = Command::new("ping")
        .arg("-c")
        .arg("4")
        .arg(ip_str.clone())
        .output()
        .map_err(|e| format!("Failed to execute ping command: {}", e))?;

    // Check if the ping was successful (exit code 0 means success)
    if output.status.success() {
        // Ping was successful, no output needed
        println!("Ping successful for {}", ip_str);
        Ok(())
    } else {
        // Ping failed, print the error message
        Err(format!("Ping failed for {}: {}", ip_str, String::from_utf8_lossy(&output.stderr)))
    }
}
