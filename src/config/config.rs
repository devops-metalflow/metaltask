use serde_derive::{Deserialize, Serialize};
use serde_yaml;
use std::error::Error;
use std::fs::File;
use std::path::Path;

pub const NAME: &'static str = "metaltask";
pub const PATH: &'static str = "/tmp";
pub const VERSION: &'static str = "metaltask/version";

#[derive(Clone, Default)]
pub struct Config {
    pub config_data: ConfigData,
    pub config_file: String,
    pub listen_url: String,
    pub version_info: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct ConfigData {
    pub apiVersion: String,
    pub kind: String,
    pub metadata: MetaData,
    pub spec: Spec,
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct MetaData {
    pub name: String,
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Spec {
    pub task: Task,
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Task {
    pub clean: bool,
}

impl Config {
    pub fn build(&mut self) -> Result<(), Box<dyn Error>> {
        self.config()?;
        self.listen()?;
        self.version()?;

        Ok(())
    }

    pub fn config(&mut self) -> Result<(), Box<dyn Error>> {
        if self.config_file.len() == 0 {
            return Err("name invalid".into());
        }

        if !self.config_file.ends_with(".yml") && !self.config_file.ends_with(".yaml") {
            return Err("suffix invalid".into());
        }

        if !Path::new(&self.config_file).exists() {
            return Err(format!("{} not found", self.config_file).into());
        }

        let f = File::open(&self.config_file)?;
        self.config_data = serde_yaml::from_reader(f)?;

        Ok(())
    }

    pub fn listen(&mut self) -> Result<(), Box<dyn Error>> {
        if self.listen_url.len() == 0 {
            return Err("url invalid".into());
        }

        Ok(())
    }

    pub fn version(&mut self) -> Result<(), Box<dyn Error>> {
        // PASS
        Ok(())
    }
}
