use std::{fs, io};
use std::env;
use std::path::{Path, PathBuf};
use std::time::Duration;

use path_clean::PathClean;
use sha1::{Digest, Sha1};
use tokio::task::JoinHandle;

use crate::errors::OracleError;
use crate::errors::OracleError::CustomError;

pub async fn sleep_ms(ms: u64) {
    tokio::time::sleep(Duration::from_millis(ms)).await;
}

/// Get Sha1 hash of current executable file, panics on failure.
#[must_use]
pub fn calculate_current_executable_hash() -> String {
    let bin_path = env::current_exe().expect("Unable to get current executable path.");
    let mut file =
        fs::File::open(&bin_path).expect("Unable to locate current executable for hashing.");
    let mut hasher = Sha1::new();
    io::copy(&mut file, &mut hasher).expect("Failed to read current executable for hashing.");
    let hash = hasher.finalize();
    hex::encode(hash)
}

/// # Errors
///
/// Will return `Err` if it's been failed to get `current_dir`
pub fn absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    }
    .clean();
    Ok(absolute_path)
}

/// # Errors
///
/// Will return `OracleError` if join `handle` returns it or
/// related thread has been failed for some reason
pub async fn join_flatten<T>(handle: JoinHandle<Result<T, OracleError>>) -> Result<T, OracleError> {
    match handle.await {
        Ok(Ok(result)) => Ok(result),
        Ok(Err(err)) => Err(err),
        Err(err) => Err(CustomError(format!("handling failed: {}", err))),
    }
}
