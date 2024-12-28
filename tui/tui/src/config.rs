use admin::apis::configuration::Configuration;
use std::ops::Deref;

use crate::credentials::RillCredentails;

#[derive(Debug, Clone)]
pub struct ClientConfig(Configuration);

impl Deref for ClientConfig {
    type Target = Configuration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        let base_path = std::env::var("RILL_BASE_PATH")
            .unwrap_or_else(|_| "https://admin.rilldata.in".to_string());
        let credentials = RillCredentails::default();
        Self {
            0: Configuration {
                base_path,
                api_key: Some(credentials.api_key("Bearer".to_owned())),
                ..Default::default()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let config = ClientConfig::default();
        assert_eq!(config.base_path, "https://admin.rilldata.in");
        assert_eq!(
            config.api_key.as_ref().and_then(|key| key.prefix.as_ref()),
            Some(&"Bearer".to_string())
        );
    }

    fn test_default_with_env() {
        std::env::set_var("RILL_BASE_PATH", "http://test.path.com");
        let config = ClientConfig::default();
        assert_eq!(config.base_path, "http://test.path.com");
    }
}
