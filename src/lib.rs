use ports::TTLInputPort;
use utils::result::AppResult;

mod assembler;
pub mod infras;
pub mod ports;
pub mod utils;

pub struct App<T: TTLInputPort> {
    ttl_input_port: T,
}

impl<T: TTLInputPort> App<T> {
    pub fn new(input_port: T) -> Self {
        App {
            ttl_input_port: input_port,
        }
    }

    pub fn assemble(&self, path: &str) -> AppResult<()> {
        println!("Assembling from fg{}...", path);
        let file = self.ttl_input_port.read(path)?;

        Ok(())
    }
}
