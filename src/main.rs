#![feature(fs_try_exists)]
#![feature(is_some_with)]

use chrono::{FixedOffset, TimeZone, Utc};
use clap::{ArgGroup, Args, Parser, Subcommand};
use prettytable::Table;
use record::structure::{Config, ConfigStruct, RECORD_CONFIG_DIR, RECORD_CONFIG_FILENAME};
use std::env::home_dir;
use std::fs::{remove_file, try_exists};

use record::commands::{add, clear, view};
use record::fs::initial_config_file::initial_config_file;

#[macro_use]
extern crate prettytable;

#[derive(Parser, Debug)]
#[clap(name = "Record")]
#[clap(version = "v0.1")]
#[clap(author = "shulandmimi <shulandmimi@163.com>")]
#[clap(about = "record something")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Delete,
    Modify,
    Add(add::Add),
    View(view::View),
    Clear(clear::Clear),
}

fn main() {
    let filename = home_dir()
        .unwrap()
        .join(RECORD_CONFIG_DIR)
        .join(RECORD_CONFIG_FILENAME);
    initial_config_file(home_dir().unwrap());

    let args = Cli::parse();

    if args.command.is_none() {
        return;
    }

    let config = Config::new(filename.clone());

    args.command.map(|command| {
        match command {
            Commands::Delete => {
                println!("delete");
            }

            Commands::Modify => {
                println!("modify");
            }

            Commands::Add(cmd) => {
                cmd.run(config);
            }

            Commands::View(cmd) => {
                cmd.run(config);
            }

            Commands::Clear(cmd) => {
                cmd.run(config);
            }
        };
    });
}
