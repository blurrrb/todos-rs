use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}
