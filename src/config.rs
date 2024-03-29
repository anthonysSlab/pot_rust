use std::{env, fs};

use serde::Deserialize;
use once_cell::sync::Lazy;
use std::sync::Arc;
use serenity::model::prelude::UserId;

pub static CONFIG: Lazy<Conf> = Lazy::new(Conf::init);

// struct to load the config into
#[derive(Deserialize)]
pub struct Conf {
    pub token_file:  String,
    pub prefix:      char,
    pub permissions: Perms,
    pub status:      Stat,
    pub replies:     Reply,
}

#[derive(Deserialize, Clone)]
pub struct Perms {
    pub ownr:    Arc<[UserId]>,
    pub modr:    Arc<[UserId]>,
    pub replies: Vec<String>,
}

// config struct for statuses
#[derive(Deserialize, Clone)]
pub struct Stat {
    pub enable:       bool,
    pub status_delay: u16,
    pub randomize:    bool,
    pub status_list:  Vec<String>,
}

#[derive(Deserialize)]
pub struct Reply {
    pub enable:     bool,
    pub chance:     u8,
    pub iterations: u8,
    pub url_block:  bool,
    pub blacklist:  Arc<[String]>,
    pub list:       Vec<String>,
    pub trigger:    Arc<[String]>,
}

impl Conf {
    fn init() -> Self {
        // check env var, if empty pick the default
        let config_file = env::var("POT_CONFIG")
            .unwrap_or("config.toml".to_string());

        // load from a file
        let contents = fs::read_to_string(config_file)
            .expect("Failed to read config");
            
        // return the parsed struct
        toml::from_str::<Conf>(&contents)
            .expect("Failed to Parse Config")
    }
}

