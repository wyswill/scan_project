use project_info::FileManager;
use std::env;
mod config;
mod project_info;

fn main() {
    let _config_path: String = get_config_path();
    let _conf_ins: config::Config = config::Config::init_form_json(&_config_path);
    let mut _project_manager: FileManager = FileManager::new(_conf_ins);
    _project_manager.scan_dir();
    _project_manager.fmt();
    _project_manager.write_config();
}

fn get_config_path() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("config file has not set")
    }
    if let Some(path) = args.get(1) {
        return path.to_string();
    } else {
        return "".to_string();
    }
}
