use std::{fmt::Debug, time::Duration};

use confy::{self, ConfyError};
use ftp::FtpStream;
use serde::{Deserialize, Serialize};

static CONFIG_FILE_NAME: &str = "klikbox-ftp-client";

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    ip: String,
    src: String,
    dst: String,
}

impl Config {
    fn load() -> Result<Config, String> {
        let cfg = match confy::load(CONFIG_FILE_NAME) {
            Ok(config) => config,
            error => return Err(format!("Failed to read config file: {:?}", error)),
        };
        println!("cfg: {:?}", cfg);
        match confy::store(CONFIG_FILE_NAME, &cfg) {
            Ok(_) => Ok(cfg),
            error => return Err(format!("Failed to write config file: {:?}", error)),
        }
    }
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1:21".to_string(),
            src: "/".to_string(),
            dst: "c:\\temp".to_string(),
        }
    }
}

struct Connection {
    stream: FtpStream,
}

impl Connection {
    fn new(ip: &str) -> Result<Self, String> {
        match FtpStream::connect(ip) {
            Ok(stream) => return Ok(Connection { stream }),
            error => Err(format!("Failed to connect to {}: {:?}", ip, error)),
        }
    }
}

fn main() {
    run();

    // println!("Connected!");
    // ftp_stream
    //     .get_ref()
    //     .set_read_timeout(Some(Duration::from_secs(10)))
    //     .expect("set_read_timeout call failed");
    // ftp_stream.login("Doe", "mumble").unwrap();
    // println!("Logged in!");
    // let cwd = ftp_stream.pwd().unwrap();
    // println!("CD: {:?}", cwd);
    // let list = ftp_stream.nlst(Some(&cwd)).unwrap();
    // println!("Dir: {:?}!", &list);
    // let _ = ftp_stream.quit();
}

fn run() -> Result<(), String> {
    let cfg = Config::load()?;
    Ok(())
}
