mod log;

use sysinfo::System;
use log::{Log, LogLevel};

fn get_system_info() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();
    let cpu_usage = sys.global_cpu_usage();
    let memory_used = sys.used_memory();
    let memory_total = sys.total_memory();
    
    if memory_used > memory_total * 90 / 100 {
        clear_cache();
    }

    format!(
        "CPU Usage: {:.2}% | Memory Used: {} GB / {} GB",
        cpu_usage,
        memory_used / 1048576,
        memory_total / 1048576
    )
}

use std::process::Command;
fn clear_cache() {
    let mut sys = System::new_all();
    sys.refresh_all();
    let _ = Command::new("sh")
        .arg("-c")
        .arg("echo 3 > /proc/sys/vm/drop_caches")
        .status();
}

use std::fs;
fn get_temperatures() -> String {

    let output = Command::new("sensors").output().expect("Failed to execture `sensors` command");
    let output_str = String::from_utf8_lossy(&output.stdout);

    for line in output_str.lines() {
        if line.trim().starts_with("Tctl:") {
            if let Some(temp) = line.split_whitespace().nth(1) {
                return format!("CPU Temperature: {:.1}°C", temp);
            }
        }
    }
    "Temperature data not available".to_string()
}

#[tokio::main]
async fn main() {
    println!("Starting application!");

    let mut log_file = Log::init();
    log_file.write_to_log(LogLevel::Debug, "Application started!");
        
    ctrlc::set_handler(move || {
        println!("Shutting down...");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    loop {
        let system_info = get_system_info();
        println!("{}", system_info);
        let temperatures = get_temperatures();
        println!("{}", temperatures);

        // TODO: implementare ping a lista di servizi di rete specificati

        // TODO: implement MQTT message sending
        // send_mqtt_message("system/info", &system_info).await;

        log_file.write_to_log(LogLevel::Debug, "log file test");
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
}

