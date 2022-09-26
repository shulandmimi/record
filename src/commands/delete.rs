use clap::Args;

use crate::structure::{Config, ConfigStruct, Message};

#[derive(Debug, Args)]
pub struct Delete {
    #[clap(long)]
    hash: String,
}

impl Delete {
    pub fn run(&self, config: Config) {
        let mut config_struct = ConfigStruct::from_file(&config.config_file).unwrap();

        let new_datas: Vec<Message> = config_struct
            .datas
            .into_iter()
            .filter(|item| !item.hash.starts_with(&self.hash))
            .collect();

        config_struct.datas = new_datas;

        config_struct.to_file(&config.config_file).unwrap();
    }
}
