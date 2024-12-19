use sysinfo::System;

fn get_system_info() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();
    let cpu_usage = sys.global_cpu_usage();
    let memory_used = sys.used_memory();
    let memory_total = sys.total_memory();
    format!(
        "CPU Usage: {:.2}% | Memory Used: {} MB / {} MB",
        cpu_usage,
        memory_used / 1024,
        memory_total / 1024
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
    let temp_path = "/sys/class/thermal/thermal_zone0/temp";
    if let Ok(content) = fs::read_to_string(temp_path) {
        let temp = content.trim().parse::<f64>().unwrap_or(0.0) / 1000.0;
        return format!("CPU Temperature: {:.1}°C", temp);
    }
    "Temperature data not available".to_string()
}

#[tokio::main]
async fn main() {
    println!("Starting application!");
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
        // send_mqtt_message("system/info", &system_info).await;
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}

