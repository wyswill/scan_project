use config::Config;
use project_info::FileManager;
use std::env;
mod config;
mod project_info;

fn main() {
    let config_path: String = get_config_path();
    let conf_ins: Config = Config::init_form_json(&config_path);
    let mut project_manager: FileManager = FileManager::new(conf_ins);
    project_manager.scan_dir();
    project_manager.fmt();
    project_manager.write_config();
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
