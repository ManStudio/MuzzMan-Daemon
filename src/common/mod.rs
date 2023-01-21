#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

use std::path::PathBuf;

#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "windows")]
pub use windows::*;

pub fn get_modules() -> Vec<PathBuf> {
    let mut modules = Vec::new();
    for paths in get_muzzman_dir().read_dir().unwrap() {
        let entry = paths.unwrap();
        let path = entry.path();
        if path
            .as_os_str()
            .to_string_lossy()
            .split('.')
            .last()
            .unwrap()
            == library_termination()
        {
            modules.push(path);
        }
    }
    modules
}
