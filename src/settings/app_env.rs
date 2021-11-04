use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub enum ApplicationEnvironment {
    Development,
    Production,
}

impl fmt::Display for ApplicationEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApplicationEnvironment::Development => write!(f, "development"),
            ApplicationEnvironment::Production => write!(f, "production"),
        }
    }
}
