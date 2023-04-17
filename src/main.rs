use serde::{Deserialize, Serialize};
use std::{borrow::BorrowMut, fs};

#[derive(Debug, Serialize, Deserialize)]
struct ProjectInfo {
    name: String,
    rootPath: String,
    paths: Vec<String>,
    tags: Vec<String>,
    enabled: bool,
}

fn main() {
    let _config_path: &str = "/Users/wangyansong/Library/Application Support/Code/User/globalStorage/alefragnani.project-manager/projects.json";
    let vw_dir: &str = "/Users/wangyansong/project/vw";
    let cooper_dir: &str = "/Users/wangyansong/project/cooper";
    let mut info_list: Vec<ProjectInfo> = Vec::new();
    scan_dir(cooper_dir, "cooper", info_list.borrow_mut());
    scan_dir(vw_dir, "vw", info_list.borrow_mut());
    println!("{:#?}", info_list);
    let json_string: String = serde_json::to_string(&info_list).unwrap();
    fs::write(_config_path, json_string).expect("写入失败");
    println!("写入成功");
}

fn scan_dir(_path: &str, tag: &str, info_list: &mut Vec<ProjectInfo>) {
    if let Ok(dir) = fs::read_dir(_path) {
        for entry in dir {
            let _dir: fs::DirEntry = entry.unwrap();
            let path: String = _dir.path().to_str().unwrap().to_string();
            if path.contains(".DS_Store") {
                continue;
            }
            let name: String = path.replace(_path, "").replace("/", "");
            let tags: Vec<String> = vec![tag.to_string()];
            let info: ProjectInfo = ProjectInfo {
                name,
                rootPath: path,
                paths: Vec::new(),
                tags,
                enabled: true,
            };
            info_list.push(info);
        }
    }
}
