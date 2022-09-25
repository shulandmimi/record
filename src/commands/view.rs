use crate::{
    structure::{Config, ConfigStruct, Message},
    util::hash,
};
use chrono::{TimeZone, Utc, FixedOffset};
use clap::{ArgGroup, Args};
use prettytable::{row, Table};

#[derive(Debug, Args)]
#[clap(group(ArgGroup::new("View")))]
pub struct View {
    #[clap(long, action(clap::ArgAction::SetTrue), default_value_t = false)]
    verbose: bool,
}

impl View {
    pub fn run(&self, config: Config) {
        let mut config_struct = ConfigStruct::from_file(&config.config_file).unwrap();

        if self.verbose {
            let mut table = Table::new();

            table.add_row(row!["ID", "Message", "CreateTime"]);

            config_struct.datas.iter().for_each(|item| {
                let china_timezone = FixedOffset::east(8 * 3600);
                table.add_row(row![
                    item.hash,
                    item.message,
                    Utc.timestamp(item.c_time, 0)
                        .with_timezone(&china_timezone)
                        .to_string()
                ]);
            });

            table
                .print_tty(true)
                .expect("print messages error, please report to author");
        } else {
            config_struct.datas.sort_by(|a, b| b.c_time.cmp(&a.c_time));

            config_struct
                .datas
                .iter()
                .for_each(|item| println!("{}", item.message));
        }
    }
}
