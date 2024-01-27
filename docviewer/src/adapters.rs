use lib_core::{
    ports::ConfigProviderPort,
    result::{AppError, AppResult},
};

pub struct ConfigFromFile {
    layers: Vec<String>,
}
impl ConfigFromFile {
    pub fn new(resolution_dir: &str) -> AppResult<Self> {
        let layers =
            std::fs::read_to_string(format!("{}/layers.csv", resolution_dir)).map_err(|e| {
                AppError::String(format!(
                    "Could not read layers.csv from resolution directory: {}",
                    e
                ))
            })?;

        let layers = layers.split(',').map(|s| s.to_string()).collect();

        Ok(Self { layers })
    }
}
impl ConfigProviderPort for ConfigFromFile {
    fn get_transform_layers(&self) -> AppResult<Vec<String>> {
        Ok(self.layers.clone())
    }
}
