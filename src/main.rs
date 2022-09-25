#![feature(fs_try_exists)]
#![feature(is_some_with)]

use base64ct::{Base64, Encoding};
use clap::{ArgGroup, Args, Parser, Subcommand};
use prettytable::{Table};
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::env::{home_dir};
use std::fs::{create_dir_all, read, remove_file, try_exists, write};
use std::path::PathBuf;
use chrono::{FixedOffset, TimeZone, Utc};

fn hash(data: &String) -> String {
    let mut sh = Sha1::new();

    sh.update(data);

    let result = sh.finalize();

    return Base64::encode_string(&result);
}

#[macro_use]
extern crate prettytable;

const RECORD_CONFIG_DIR: &str = ".config/record";

const RECORD_CONFIG_FILENAME: &str = "record.json";

fn initial_config_file(dir: PathBuf) {
    // let user_home = home_dir().unwrap();

    let dirname = dir.join(RECORD_CONFIG_DIR);
    let filename = dirname.clone().join(RECORD_CONFIG_FILENAME);

    if try_exists(&filename).ok().unwrap() {
        return;
    };

    if !try_exists(&dirname).ok().unwrap() {
        create_dir_all(&dirname).expect("create dir failed");
    }

    println!("{}", filename.to_string_lossy());

    if let Err(err) = write(&filename, "") {
        panic!("Error: {}\n\nnot create config, please retry", err);
    } else {
        println!(
            "{} created on {}",
            RECORD_CONFIG_FILENAME,
            dirname.to_str().unwrap()
        );
    };
}

#[derive(Parser, Debug)]
#[clap(name = "Record")]
#[clap(version = "v0.1")]
#[clap(author = "shulandmimi <shulandmimi@163.com>")]
#[clap(about = "record something")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser, Debug)]
struct Delete {
    #[clap(long, value_parser)]
    id: String,
}

#[derive(Debug, Args)]
#[clap(group(ArgGroup::new("Add")))]
struct Add {
    #[clap(long, short)]
    message: String,
}

#[derive(Debug, Args)]
#[clap(group(ArgGroup::new("View")))]
struct View {
    #[clap(long, action(clap::ArgAction::SetTrue), default_value_t = false)]
    verbose: bool,
}

#[derive(Debug, Args)]
#[clap(group(ArgGroup::new("Clear")))]
struct Clear {}

#[derive(Subcommand, Debug)]
enum Commands {
    Delete,
    Modify,
    Add(Add),
    View(View),
    Clear(Clear),
}

#[derive(Deserialize, Serialize, Debug)]
struct ConfigStruct {
    datas: Vec<Message>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Message {
    hash: String,
    message: String,
    c_time: i64,
}

impl ConfigStruct {
    fn new() -> Self {
        ConfigStruct { datas: Vec::new() }
    }
    fn from_file(filename: &PathBuf) -> Result<Self, String> {
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

    fn to_file(&self, filename: &PathBuf) -> Result<bool, String> {
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

    args.command.map(|command| {
        match command {
            Commands::Delete => {
                println!("delete");
                let arg = Delete::parse();
                println!("{:?}", arg);
            }
            Commands::Modify => {
                println!("modify");
            }
            Commands::Add(cmd) => {
                let cur = Utc::now().timestamp();
                let message = cmd.message;

                let item = Message {
                    c_time: cur,
                    hash: hash(&message),
                    message,
                };

                let mut config_struct = ConfigStruct::from_file(&filename).unwrap();
                config_struct.datas.push(item);
                config_struct.to_file(&filename);
            }
            Commands::View(cmd) => {
                let mut config_struct = ConfigStruct::from_file(&filename).unwrap();

                if cmd.verbose {
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

                    table.print_tty(true);
                } else {
                    config_struct.datas.sort_by(|a, b| b.c_time.cmp(&a.c_time));

                    config_struct
                        .datas
                        .iter()
                        .for_each(|item| println!("{}", item.message));
                }
            }
            Commands::Clear(cmd) => {
                if try_exists(&filename).is_ok_and(|ok| *ok) {
                    remove_file(&filename)
                        .expect(format!("remove {} failed", &filename.to_str().unwrap()).as_str());
                } else {
                    println!("{} not exists", &filename.to_str().unwrap())
                }
            }
        };
    });
}
