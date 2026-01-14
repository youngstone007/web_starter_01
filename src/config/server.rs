use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct ServerConfig {
    port: Option<u32>,
}

impl ServerConfig {
    pub fn port(&self) -> u32 {
        self.port.unwrap_or(3000)
    }
}
