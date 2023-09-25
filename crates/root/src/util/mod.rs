use core::result::Result;
use std::env;
use std::env::VarError;

pub fn unwrap_join_all<I, V>(collection: I) -> Result<V, VarError>
where
    I: IntoIterator,
    I::Item: FnOnce() -> core::result::Result<V, VarError>,
{
    let iter = collection.into_iter();

    for item in iter {
        if let Ok(val) = item() {
            return Ok(val);
        }
    }

    return Err(VarError::NotPresent);
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn get_home_dir() -> Result<String, VarError> {
    unwrap_join_all([|| env::var("HOME"), || env::var("home")])
}

#[cfg(target_os = "windows")]
pub fn get_home_dir() -> Result<String, VarError> {
    unwrap_join_all([|| env::var("USERPROFILE"), || env::var("HOME")])
}
