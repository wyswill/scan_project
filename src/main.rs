use serde::{Deserialize, Serialize};
use std::{borrow::BorrowMut, collections::HashSet, fs};

#[derive(Debug, Serialize, Deserialize)]
struct ProjectInfo {
    name: String,
    root_path: String,
    paths: Vec<String>,
    tags: Vec<String>,
    enabled: bool,
}

fn main() {
    let mut info_list: Vec<ProjectInfo> = Vec::new();
    let mut dic: HashSet<String> = HashSet::new();
    let _config_path: &str = "/Users/wangyansong/Library/Application Support/Code/User/globalStorage/alefragnani.project-manager/projects.json";
    let vw_dir: &str = "/Users/wangyansong/project/vw";
    let cooper_dist: Vec<&str> = vec![
        "/Users/wangyansong/project/cooper/rs_tools",
        "/Users/wangyansong/project/cooper",
    ];
    for _cooper_path in cooper_dist.iter() {
        scan_dir(_cooper_path, "cooper", &mut info_list, &mut dic);
    }
    scan_dir(vw_dir, "vw", &mut info_list, &mut dic);
    println!("{:#?}", info_list);
    // let json_string: String = serde_json::to_string(&info_list).unwrap();
    // fs::write(_config_path, json_string).expect("写入失败");
    // println!("写入成功");
}

fn scan_dir(_path: &str, tag: &str, info_list: &mut Vec<ProjectInfo>, set: &mut HashSet<String>) {
    if let Ok(dir) = fs::read_dir(_path) {
        for entry in dir {
            let _dir: fs::DirEntry = entry.unwrap();
            let root_path: String = _dir.path().to_str().unwrap().to_string();
            if root_path.contains(".") {
                continue;
            }
            let name: String = get_end_path(&root_path);
            if set.contains(&name) {
                continue;
            }
            set.insert(name.clone());
            let tags: Vec<String> = vec![tag.to_string()];
            let info: ProjectInfo = ProjectInfo {
                name,
                root_path: root_path,
                paths: Vec::new(),
                tags,
                enabled: true,
            };
            info_list.push(info);
        }
    }
}

fn get_end_path(path: &str) -> String {
    let arr = path.split('/');
    if let Some(res) = arr.last() {
        return res.to_string();
    } else {
        return "".to_string();
    }
}
