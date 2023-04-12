use muzzman_daemon::{
    common::get_muzzman_dir, daemon::Daemon, prelude::TModuleInfo, DaemonSession,
};

fn main() {
    env_logger::init();

    let daemon = Daemon::new().unwrap();
    let daemon = std::thread::spawn(|| daemon.run());
    {
        let session = DaemonSession::new()
            .expect("Some thing went rong!")
            .create_session();
        if let Ok(dir) = get_muzzman_dir().join("modules").read_dir() {
            for file in dir {
                if let Ok(file) = file {
                    let path = file.path();
                    if let Some(name) = file.file_name().to_str() {
                        if name.contains(std::env::consts::DLL_EXTENSION) {
                            match session.load_module(path.clone()) {
                                Ok(module) => {
                                    println!("Loaded module: {}", module.get_name().unwrap());
                                }
                                Err(err) => {
                                    eprintln!("Error when loading: {:?}\n{:?}\n\n", path, err);
                                }
                            }
                        }
                    } else {
                        eprintln!("Cannot get the file_name for {:?}", path)
                    }
                } else {
                    eprintln!("Cannot get the file");
                }
            }
        }
    }

    daemon.join().unwrap();
}
