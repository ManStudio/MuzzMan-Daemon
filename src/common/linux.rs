use std::{ffi::OsString, path::PathBuf};

pub fn get_muzzman_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap()
        .join(".local")
        .join("share")
        .join("MuzzMan")
}

pub fn library_termination() -> OsString {
    "so".into()
}
