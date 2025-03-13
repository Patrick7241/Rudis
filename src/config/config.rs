use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rudis: RudisConfig,
}

#[derive(Debug, Deserialize)]
pub struct RudisConfig {
    pub address : String,
}