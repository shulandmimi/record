use std::fs::{create_dir_all, try_exists, write};
use std::path::PathBuf;

const RECORD_CONFIG_DIR: &str = ".config/record";

const RECORD_CONFIG_FILENAME: &str = "record.json";

pub fn initial_config_file(dir: PathBuf) {
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
