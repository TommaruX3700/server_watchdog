mod plugin_api;
use plugin_api::Plugin;

use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

/*
    TODO:
        - test if viable solution
        - test released product
        - evaluate if simplify project and add other components in a second time
        - evaluate to develop all the single components and then integrate them toghether
 */

fn main() {
    let mut plugins: HashMap<String, Box<dyn Plugin>> = HashMap::new();
    let plugin_root_dir = "src/std_components";

    // Ciclo principale per caricare ed eseguire i plugin
    loop {
        // Scansiona ricorsivamente il root dei plugin
        println!("Scansione in corso..");
        if let Ok(entries) = fs::read_dir(plugin_root_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    // Cerca librerie dinamiche nella sottodirectory
                    if let Ok(files) = fs::read_dir(&path) {
                        for file in files.flatten() {
                            let file_path = file.path();
                            if file_path.extension().and_then(|ext| ext.to_str()) == Some("so") {
                                let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap().to_string();
                                if !plugins.contains_key(&file_name) {
                                    match load_plugin(&file_path) {
                                        Ok(plugin) => {
                                            println!("Caricato plugin: {}", plugin.name());
                                            plugins.insert(file_name, plugin);
                                        }
                                        Err(e) => eprintln!("Errore durante il caricamento del plugin: {}", e),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Esegue tutti i plugin caricati
        for plugin in plugins.values() {
            plugin.execute();
        }

        thread::sleep(Duration::from_secs(5));
    }
}

fn load_plugin(path: &Path) -> Result<Box<dyn Plugin + 'static>, String> {
    // Unsafe block for loading the library
    let lib = unsafe {
        Library::new(path).map_err(|e| format!("Impossibile caricare il file: {}", e))?
    };

    unsafe {
        // We load the 'create_plugin' symbol, which should return a Box<dyn Plugin>
        let constructor: Symbol<extern "C" fn() -> Box<dyn Plugin + 'static>> =
            lib.get(b"create_plugin")
                .map_err(|e| format!("Impossibile trovare la funzione: {}", e))?;
        
        // Call the constructor to get the Plugin
        Ok(constructor())
    }
}