use super::errors::ConfigError;
use rtmp::session::server_session::RtmpWebhookConfig;
use serde_derive::Deserialize;
use std::fs;
use std::vec::Vec;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rtmp: Option<RtmpConfig>,
    pub httpflv: Option<HttpFlvConfig>,
    pub hls: Option<HlsConfig>,
    pub log: Option<LogConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RtmpConfig {
    pub enabled: bool,
    pub port: u32,
    pub pull: Option<RtmpPullConfig>,
    pub push: Option<Vec<RtmpPushConfig>>,
    pub webhooks: Option<RtmpWebhookConfig>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct RtmpPullConfig {
    pub enabled: bool,
    pub address: String,
    pub port: u16,
}
#[derive(Debug, Deserialize, Clone)]
pub struct RtmpPushConfig {
    pub enabled: bool,
    pub address: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HttpFlvConfig {
    pub enabled: bool,
    pub port: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HlsConfig {
    pub enabled: bool,
    pub port: u32,
}

pub enum LogLevel {
    Info,
    Warn,
    Error,
    Trace,
    Debug,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LogConfig {
    pub level: String,
}

pub fn load(cfg_path: &String) -> Result<Config, ConfigError> {
    let content = fs::read_to_string(cfg_path)?;
    let decoded_config = toml::from_str(&content[..]).unwrap();
    Ok(decoded_config)
}

#[test]
fn test_toml_parse() {
    // let path = env::current_dir();
    // match path {
    //     Ok(val) => println!("The current directory is {}\n", val.display()),
    //     Err(err) => print!("{}\n", err),
    // }

    let str = fs::read_to_string("/Users/zexu/github/gms/application/src/config/config.toml");

    match str {
        Ok(val) => {
            println!("++++++{val}\n");
            let decoded: Config = toml::from_str(&val[..]).unwrap();

            let rtmp = decoded.rtmp;
            if let Some(val) = rtmp {
                println!("++++++{}\n", val.enabled);
            }
        }
        Err(err) => println!("======{err}"),
    }
}
