use crate::config::config::{Config, NAME};
use chrono::{DateTime, Local};
use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Default)]
pub struct Task {
    pub data: Vec<u8>,
    pub path: PathBuf,
}

impl Task {
    pub fn routine(cfg: Config, data: Vec<u8>) -> Result<String, Box<dyn Error>> {
        let now: DateTime<Local> = Local::now();
        let name = now.to_rfc3339();

        let path = Path::new(&cfg.config_data.spec.task.path);
        let path = path.join(NAME.to_string() + &"-".to_string() + &name);

        let task = Task { data, path };

        task.create()?;
        task.exec()?;

        if cfg.config_data.spec.task.clean {
            task.clean()?;
        }

        Ok("".to_string())
    }

    pub fn create(&self) -> Result<String, Box<dyn Error>> {
        let mut file = match fs::File::create(&self.path) {
            Ok(file) => file,
            Err(_) => return Err("create failed".into()),
        };

        match file.write_all(&*self.data) {
            Ok(_) => Ok("".to_string()),
            Err(_) => Err("write failed".into()),
        }
    }

    pub fn exec(&self) -> Result<String, Box<dyn Error>> {
        let output = Command::new("bash").arg(&self.path).output()?;
        if !output.status.success() {
            return Err("exec failed".into());
        }

        match String::from_utf8(output.stdout) {
            Ok(buf) => Ok(buf),
            Err(_) => Err("output invalid".into()),
        }
    }

    pub fn clean(&self) -> Result<String, Box<dyn Error>> {
        fs::remove_file(&self.path)?;
        Ok("".to_string())
    }
}
