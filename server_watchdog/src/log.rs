use std::fs::File;

struct Log {
    m_file : File
    // TODO:
    // - add buffer for errors
    // - add buffer for warnings
    // - add buffer for infos
    // - add max file size
    // - add timestamp
}

impl Log {
    pub fn init() -> Log {
        let file = File::create("server_watchdog.log").expect("Error creating log file");
        


        Log {
            m_file : file

        }
    }
}

fn write_to_file() {

}

