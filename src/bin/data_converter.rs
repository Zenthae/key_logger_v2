use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use chrono::{DateTime, Local, Utc};
use key_logger_v2::{Input, RecordHolder};
use serde_json::de::Deserializer;

const APP_ID: &str = "key_logger_v2";
const VENDOR: &str = "Zenthae";
const LOG_FILE_NAME: &str = "records.log";

fn main() {
    let project_dirs = directories_next::ProjectDirs::from("", VENDOR, APP_ID)
        .expect("Can't find a valid home directory");

    let mut path = project_dirs.data_dir().to_path_buf();

    path.push(LOG_FILE_NAME);

    if let Ok(log_file) = OpenOptions::new().read(true).open(path) {
        // println!("File opened !");
        let mut record_holder = RecordHolder::new();
        for (input, time) in Deserializer::from_reader(log_file)
            .into_iter::<(Input, DateTime<Utc>)>()
            .filter_map(|r| {
                // println!("{:?}", r);
                match r {
                    Ok(v) => Some(v),
                    Err(e) => {
                        // println!("{:?}", e);
                        if let Ok(mut crash_report) = File::create(format!(
                            "./crash-report-{}.txt",
                            Local::now().format("%F_%H.%M.%S")
                        )) {
                            crash_report.write(format!("{}", e).as_bytes()).unwrap();
                        };
                        None
                    }
                }
            })
        {
            // println!("{:?} - {:?}", input, time);
            record_holder.entry(input).or_insert(Vec::new()).push(time);
        }
        if let Ok(data_file) =
            File::create(format!("data-{}.json", Local::now().format("%F_%H.%M.%S")))
        {
            serde_json::to_writer_pretty(&data_file, &record_holder).expect("u mad bro ?");
        }
    }
}
