use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: u32,
}
