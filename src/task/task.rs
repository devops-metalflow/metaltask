use crate::config::config::Config;
use std::error::Error;

pub struct Task {}

impl Task {
    pub fn routine(cfg: Config, data: Vec<u8>) -> Result<String, Box<dyn Error>> {
        Ok("".to_string())
    }
}
