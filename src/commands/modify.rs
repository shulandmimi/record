use clap::Args;

use crate::{
    structure::{Config, ConfigStruct, Message},
    util::hash,
};

#[derive(Debug, Args)]
pub struct Modify {
    #[clap(long)]
    hash: String,
    #[clap(long, short)]
    message: String,
}

impl Modify {
    pub fn run(&self, config: Config) {
        let mut config_struct = ConfigStruct::from_file(&config.config_file).unwrap();

        let item = config_struct
            .datas
            .iter_mut()
            .find(|item| item.hash.starts_with(&self.hash));

        if let Some(message) = item {
            message.message = self.message.clone();
            message.hash = hash(&self.message);
            config_struct.to_file(&config.config_file).unwrap();
        } else {
            println!("{} not record", self.hash);
        }
    }
}
