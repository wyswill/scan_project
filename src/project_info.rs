use serde::{Deserialize, Serialize};
use std::{env::consts::OS, ffi, fs, str::FromStr};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub root_path: String,
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
            if path.contains(tag) {
                return vec![tag.to_string()];
            }
        }
        return Vec::new();
    }

    pub fn scan_dir(&mut self) {
        while let Some(item) = self.config_ins.scan_dir.pop() {
            if let Ok(dir) = fs::read_dir(item) {
                for entry in dir {
                    let _dir: fs::DirEntry = entry.unwrap();
                    let root_path: String = _dir.path().to_str().unwrap().to_string();
                    if root_path.contains(".") {
                        continue;
                    }
                    let name: String = self.get_end_path(&root_path);
                    if self.config_ins.ignore_dir.contains(&name) {
                        continue;
                    }

                    if self.config_ins.need_deep.contains(&name) {
                        println!("root_path {}", root_path);
                        self.config_ins.scan_dir.push(root_path.clone());
                        continue;
                    }

                    let tags: Vec<String> = self.get_tag(&root_path);
                    let info: ProjectInfo = ProjectInfo {
                        name,
                        root_path,
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
        let mut json_string: String = serde_json::to_string(&self.info_list).unwrap();
        json_string = json_string.replace("root_path", "rootPath");
        fs::write(&self.config_ins.json_dir, json_string).expect("写入失败");
        println!("写入成功");
    }
}
