use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    user: Option<String>,
    password: Option<String>,
    host: Option<String>,
    port: Option<u32>,
    database: Option<String>,
}

impl DatabaseConfig {
    pub fn host(&self) -> &str {
        self.host.as_deref().unwrap_or("127.0.0.1")
    }

    pub fn port(&self) -> u32 {
        self.port.unwrap_or(3306)
    }

    pub fn user(&self) -> &str {
        self.user.as_deref().unwrap_or("root")
    }

    pub fn password(&self) -> &str {
        self.password.as_deref().unwrap_or("root")
    }

    pub fn database(&self) -> &str {
        self.database.as_deref().unwrap_or("user")
    }
}
