use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs, io::BufReader};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub json_dir: String,
    pub scan_dir: Vec<String>,
    pub need_deep: Vec<String>,
    pub ignore_dir: Vec<String>,
    pub tags: Vec<String>,
}

impl Config {
    pub fn init_form_json(path: &str) -> Config {
        if let Ok(file) = fs::File::open(path) {
            let reader: BufReader<fs::File> = BufReader::new(file);
            let config: Config = serde_json::from_reader(reader).unwrap();
            println!("{:#?}", config);
            config
        } else {
            panic!("file read error!");
        }
    }
}
