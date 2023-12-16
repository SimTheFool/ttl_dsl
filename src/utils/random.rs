use crate::result::{AppError, AppResult};
use rand::Rng;

pub fn get_random_uf8(length: usize) -> AppResult<String> {
    let random_string = rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(length)
        .collect::<Vec<u8>>();
    let random_string = String::from_utf8(random_string).map_err(|e| {
        AppError::String(format!(
            "Error while generating random string for declaration: {}",
            e
        ))
    })?;

    Ok(random_string)
}
