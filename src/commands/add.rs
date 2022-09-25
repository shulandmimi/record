use crate::{
    structure::{Config, ConfigStruct, Message},
    util::hash,
};
use chrono::Utc;
use clap::{ArgGroup, Args};

#[derive(Debug, Args)]
#[clap(group(ArgGroup::new("Add")))]
pub struct Add {
    #[clap(long, short)]
    pub message: String,
}

impl Add {
    pub fn run(&self, config: Config) {
        let cur = Utc::now().timestamp();
        let message = self.message.clone();

        let item = Message {
            c_time: cur,
            hash: hash(&message),
            message,
        };

        let mut config_struct = ConfigStruct::from_file(&config.config_file).unwrap();
        config_struct.datas.push(item);
        config_struct
            .to_file(&config.config_file)
            .expect("save message failed, please restart");
    }
}
