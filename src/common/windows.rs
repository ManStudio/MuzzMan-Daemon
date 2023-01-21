use std::{ffi::OsString, path::PathBuf};

pub fn get_muzzman_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap()
        .join("AppData")
        .join("Local")
        .join("MuzzMan")
}

pub fn library_termination() -> OsString {
    "dll".into()
}
