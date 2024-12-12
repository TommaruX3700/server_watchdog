mod monitor;

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::io::{self, BufRead};

fn main() {
    // Create an atomic flag to signal when to stop
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    // Spawn a thread to listen for key presses
    thread::spawn(move || {
        println!("Premi 'q' per terminare il programma.");
        let stdin = io::stdin();
        let handle = stdin.lock();
        
        for line in handle.lines() {
            if let Ok(input) = line {
                if input.trim().eq_ignore_ascii_case("q") {
                    running_clone.store(false, Ordering::SeqCst); // Imposta il flag per terminare
                    break; // Esci dal ciclo di ascolto
                }
            }
        }
    });

    // Run the monitoring process
    if let Err(e) = monitor::start_monitoring(running) {
        eprintln!("Errore: {}", e);
    }

    println!("Programma terminato.");
}