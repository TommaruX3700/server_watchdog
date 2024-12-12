use notify::{Config, RecommendedWatcher, RecursiveMode, Result, Watcher, EventKind};
use std::process::Command;
use std::path::Path;
use std::sync::{mpsc::channel, Arc};
use std::sync::atomic::{AtomicBool, Ordering};

pub fn start_monitoring(running: Arc<AtomicBool>) -> Result<()> {
    let path = Path::new("./components");
    let (tx, rx) = channel();

    // Configura il watcher per monitorare la directory
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Config::default())?;
    watcher.watch(path, RecursiveMode::Recursive)?;

    println!("Monitoraggio della cartella: {}", path.display());

    while running.load(Ordering::SeqCst) { // Controlla continuamente il flag
        // Controlla gli eventi del file system
        match rx.recv_timeout(std::time::Duration::from_millis(500)) {
            Ok(Ok(event)) => {
                for path in event.paths {
                    if matches!(event.kind, EventKind::Create(_)) {
                        println!("Nuovo file rilevato: {:?}", path);
                        if path.extension().map_or(false, |ext| ext == "rs") {
                            //println!("Inzio test di compilazione");
                            compile_and_load_plugin(&path);
                        }
                    }
                }
            }
            Ok(Err(e)) => eprintln!("Errore durante il monitoraggio: {:?}", e),
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                // Timeout, il ciclo continua
            }
            Err(e) => eprintln!("Errore nella ricezione dell'evento: {:?}", e),
        }
    }

    println!("Monitoraggio terminato.");
    Ok(())
}

fn compile_and_load_plugin(path: &Path) {
    println!("Inzio test di compilazione");
    let plugin_dir = path.parent().unwrap();
    let output = Command::new("cargo")
    .args(["build", "--release"])
    .current_dir(plugin_dir)
    .output()
    .expect("Errore durante la compilazione");

    if output.status.success() {
        println!("Compilazione completata: {:?}", plugin_dir);
        let lib_path = plugin_dir.join("target/release/libplugin.dll"); // Cambia estensione in base al sistema operativo
        load_plugin(&lib_path);
    } else {
        eprintln!("Errore di compilazione: {:?}", String::from_utf8_lossy(&output.stderr));
    }
}

fn load_plugin(lib_path: &Path) {
    unsafe {
        let lib = libloading::Library::new(lib_path).expect("Impossibile caricare la libreria");
        let func: libloading::Symbol<unsafe extern "C" fn()> =
            lib.get(b"execute").expect("Funzione non trovata");
        func();
    }
}