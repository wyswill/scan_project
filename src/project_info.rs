use serde::{Deserialize, Serialize};
use std::{env::consts::OS, ffi, fs, str::FromStr};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub rootPath: String,
    pub paths: Vec<String>,
    pub tags: Vec<String>,
    pub enabled: bool,
}
#[derive(Debug)]
pub struct FileManager {
    config_ins: Config,
    info_list: Vec<ProjectInfo>,
}
impl FileManager {
    pub fn new(config_ins: Config) -> FileManager {
        FileManager {
            config_ins,
            info_list: Vec::new(),
        }
    }

    fn get_tag(&self, path: &str) -> Vec<String> {
        for tag in &self.config_ins.tags {
            if tag.eq(path) {
                return vec![path.to_string()];
            }
        }
        return Vec::new();
    }

    pub fn scan_dir(&mut self) {
        for item in self.config_ins.scan_dir.iter() {
            if let Ok(dir) = fs::read_dir(item) {
                for entry in dir {
                    let _dir: fs::DirEntry = entry.unwrap();
                    let rootPath: String = _dir.path().to_str().unwrap().to_string();
                    if rootPath.contains(".") {
                        continue;
                    }
                    let name: String = self.get_end_path(&rootPath);
                    let tags: Vec<String> = self.get_tag(&name);
                    let info: ProjectInfo = ProjectInfo {
                        name,
                        rootPath: rootPath,
                        paths: Vec::new(),
                        tags,
                        enabled: true,
                    };
                    self.info_list.push(info);
                }
            }
        }
    }

    fn get_end_path(&self, path: &str) -> String {
        let os_str: ffi::OsString = ffi::OsString::from_str(path).unwrap();
        let _p: &str = os_str.to_str().unwrap();
        let arr: std::str::Split<char>;
        match OS {
            "linux" => arr = _p.split('/'),
            "macos" => arr = _p.split('/'),
            "windows" => arr = _p.split('\\'),
            _ => arr = "".split(' '),
        }
        if let Some(res) = arr.last() {
            return res.to_string();
        } else {
            return "".to_string();
        }
    }

    pub fn fmt(&self) {
        println!("{:#?}", self.info_list);
    }

    pub fn write_config(&mut self) {
        let json_string: String = serde_json::to_string(&self.info_list).unwrap();
        fs::write(&self.config_ins.json_dir, json_string).expect("写入失败");
        println!("写入成功");
    }
}
