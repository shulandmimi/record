use crate::structure::Config;
use clap::{ArgGroup, Args};
use std::fs::{remove_file, try_exists};

#[derive(Debug, Args)]
#[clap(group(ArgGroup::new("Clear")))]
pub struct Clear {}

impl Clear {
    pub fn run(&self, config: Config) {
        if try_exists(&config.config_file).is_ok_and(|ok| *ok) {
            remove_file(&config.config_file).expect(
                format!("remove {} failed", &config.config_file.to_str().unwrap()).as_str(),
            );
        } else {
            println!("{} not exists", &config.config_file.to_str().unwrap())
        }
    }
}
