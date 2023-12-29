use lib_interpreter::result::{AppError as LibErr, AppResult as LibRes};
use wasm_bindgen::prelude::*;

pub struct AppErr(pub LibErr);
impl From<AppErr> for JsValue {
    fn from(err: AppErr) -> Self {
        err.into()
    }
}
impl From<LibErr> for AppErr {
    fn from(err: LibErr) -> Self {
        Self(err)
    }
}

pub fn into_app_result<T>(res: LibRes<T>) -> Result<T, AppErr> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => Err(e.into()),
    }
}
