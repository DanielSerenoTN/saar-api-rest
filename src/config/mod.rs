use std::env;

pub struct GlobalConfig {
    pub addrs: String,
    pub app_name: String,
}

impl GlobalConfig {
    pub fn from_env() -> Self {
        Self {
            addrs: env::var("ADDRS").unwrap_or_else(|_| "127.0.0.1:8080".into()),
            app_name: env::var("APP_NAME").unwrap_or_else(|_| "saar_api".into()),
        }
    }
}