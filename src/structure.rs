use serde::{Deserialize, Serialize};
use std::{
    fs::{read, try_exists, write},
    path::PathBuf,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    pub hash: String,
    pub message: String,
    pub c_time: i64,
}

pub const RECORD_CONFIG_DIR: &str = ".config/record";

pub const RECORD_CONFIG_FILENAME: &str = "record.json";

#[derive(Deserialize, Serialize, Debug)]
pub struct ConfigStruct {
    pub datas: Vec<Message>,
}

impl ConfigStruct {
    pub fn from_file(filename: &PathBuf) -> Result<Self, String> {
        if !try_exists(&filename).is_ok_and(|ok| *ok) {
            return Err(format!("{} not exists", filename.to_str().unwrap()));
        };
        let s = read(filename).unwrap();

        let content: Result<String, _> = String::from_utf8_lossy(&s).parse();

        if content.is_ok_and(|c| c.is_empty()) {
            return Ok(ConfigStruct { datas: vec![] });
        }

        let config = serde_json::from_str::<ConfigStruct>(content.unwrap().as_str());

        if config.is_ok() {
            return Ok(config.unwrap());
        }

        return Err(config.err().unwrap().to_string());
    }

    pub fn new() -> Self {
        ConfigStruct { datas: Vec::new() }
    }

    pub fn to_file(&self, filename: &PathBuf) -> Result<bool, String> {
        if !try_exists(&filename).is_ok_and(|ok| *ok) {
            return Err(format!("{} not exists", filename.to_str().unwrap()));
        };

        let s = write(filename, serde_json::json!(self).to_string());

        if s.is_err() {
            return Err(s.err().unwrap().to_string());
        }

        return Ok(true);
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub config_file: PathBuf,
}

impl Config {
    pub fn new(config_file: PathBuf) -> Self {
        Config {
            config_file: config_file,
        }
    }
}
