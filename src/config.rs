use dirs;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub json_dir: String,
    pub scan_dir: Vec<String>,
    pub need_deep: Vec<String>,
    pub ignore_dir: Vec<String>,
}

impl Config {
    pub fn init_form_json(json_string: &str) -> Config {
        let data: Config = serde_json::from_str(json_string).expect("解析失败");
        return data;
    }
    /**
     * get cofnig json form defaul path
     */
    pub fn get_config_from_dir(&self) {
        let home_dir = self.get_config_from_dir();
        


    }

    fn get_home_dir() -> String {
        if let Some(home_path) = dirs::home_dir() {
            return home_path.to_str().unwrap().to_string();
        } else {
            return "".to_string();
        }
    }
}
