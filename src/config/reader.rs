use std::fs;
use serde_yaml;
use crate::config::config::Config;

pub fn reader() ->Option<Config> {
    // 读取 YAML 文件内容
    let yaml_content = match fs::read_to_string("config.yaml") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read config.yaml: {}", e);
            return None;
        }
    };

    // 解析 YAML 内容为 Rust 数据结构
    let config: Config = match serde_yaml::from_str(&yaml_content) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to parse YAML: {}", e);
            return None;
        }
    };
    Some(config)
}