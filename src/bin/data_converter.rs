use std::{
    env,
    fs::{self, OpenOptions},
    path::PathBuf,
};

use chrono::{DateTime, Utc};
use key_logger_v2::Input;
use serde_json::de::Deserializer;

// const APP_NAME: &str = "Key Logger";
// const APP_VERSION: &str = "2";
const APP_ID: &str = "key_logger_v2";
const VENDOR: &str = "Zenthae";
const LOG_FILE_NAME: &str = "records.log";

fn main() {
    let project_dirs = directories_next::ProjectDirs::from("", VENDOR, APP_ID)
        .expect("Can't find a valid home directory");

    let mut path = project_dirs.data_dir().to_path_buf();

    path.push(LOG_FILE_NAME);

    if let Ok(log_file) = OpenOptions::new().read(true).open(path) {
        println!("File opened !");
        for (input, time) in Deserializer::from_reader(log_file)
            .into_iter::<(Input, DateTime<Utc>)>()
            .filter_map(|r| {
                println!("{:?}", r);
                r.ok()
            })
        {
            println!("reading line");
            println!("{:?} - {:?}", input, time);
        }
    }
}
