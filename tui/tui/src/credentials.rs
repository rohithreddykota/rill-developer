use admin::apis::configuration::ApiKey;
use serde::{self, Deserialize};
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

type YamlKV = HashMap<String, Value>;

#[derive(Debug, Deserialize)]
struct Credentails {
    #[serde(flatten)]
    kv: YamlKV,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RillCredentails {
    pub backup_token: String,
    pub representing_user: String,
    pub token: String,
}

impl Default for RillCredentails {
    fn default() -> Self {
        // get home directory
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());

        // load credentails from ~/.rill/credentails.yaml
        let credentails_file = File::open(format!("{}/.rill/credentials.yaml", home)).unwrap();

        // load creds as key value pairs
        let reader = BufReader::new(credentails_file);
        let credentails: Credentails =
            serde_yaml::from_reader(reader).expect("failed to deser credentails.yaml");
        let kv = credentails.kv;

        // load Rill credentails
        let representing_user = kv["representing_user"].as_str().unwrap().to_string();
        let backup_token = kv["backup_token"].as_str().unwrap().to_string();
        // load token based on ENVIRONEMNT
        let environment = std::env::var("ENVIRONEMNT").ok();
        let token = if let Some(env) = environment {
            if env != "" {
                kv[format!("token.{}", env).as_str()]
                    .as_str()
                    .unwrap()
                    .to_string()
            } else {
                panic!("ENVIRONEMNT is not set");
            }
        } else {
            kv["token"].as_str().unwrap().to_string()
        };

        Self {
            backup_token,
            representing_user,
            token,
        }
    }
}

impl RillCredentails {
    pub fn api_key(&self, prefix: String) -> ApiKey {
        ApiKey {
            prefix: Some(prefix),
            key: self.token.clone(),
        }
    }
}
