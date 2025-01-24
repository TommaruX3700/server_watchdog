use core::time;
use std::{fs::File, io::Write};
use chrono::{self, DateTime, Local};

pub enum LogLevel {
    Error,
    Warning,
    Debug,
    Info
}

pub struct Log {
    // TODO:
    // - add buffer for errors
    // - add buffer for warnings
    // - add buffer for infos
    // - add max file size
    m_file : File,
}

impl Log {
    pub fn init() -> Log {
        // Dichiarare file di log e setting di base
        let mut file = File::create("server_watchdog.log").expect("Error creating log file");
        let timestamp = chrono::Local::now();
        let init_message = format!("File created at: [{}] \n", timestamp);
        file.write_all(init_message.as_bytes()).expect("Error writing to log file!");

        Log {
            m_file : file,
        }
    }

    pub fn write_to_log(&mut self, level : LogLevel, message : &str) {
        // Scrive messaggio nel file di log
        let timestamp = chrono::Local::now();

        let mut m_level = "";
        match level {
            LogLevel::Info => m_level = "INFO",
            LogLevel::Warning => m_level = "WARNING",
            LogLevel::Error => m_level = "ERROR",
            LogLevel::Debug => m_level = "DEBUG"
        }

        let log_message = format!("[{}] - [{}] {}\n", timestamp, m_level, message);
        self.m_file.write_all(log_message.as_bytes()).expect("Error writing to log file! Check permissions");
    }
}


