use std::fmt::Debug;
use std::thread;
use std::time::Duration;
use std::{error::Error, fs};

use confy::{self};
use ftp::FtpStream;
use serde::{Deserialize, Serialize};

static CONFIG_FILE_NAME: &str = "klikbox-ftp-client";

type Res<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    ip: String,
    src: String,
    dst: String,
    user: String,
    password: String,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1:21".to_string(),
            src: "/".to_string(),
            dst: "c:\\temp".to_string(),
            user: "Doe".to_string(),
            password: "mumble".to_string(),
        }
    }
}

impl Config {
    fn load() -> Res<Config> {
        let cfg: Config = confy::load(CONFIG_FILE_NAME)?;
        println!("cfg: {:?}", cfg);
        confy::store(CONFIG_FILE_NAME, &cfg)?;
        Ok(cfg)
    }
}

// struct Connection {
//     stream: FtpStream,
// }

// impl Connection {
//     fn new(ip: &str) -> Res<Self> {
//       let stream = FtpStream::connect(ip)?;
//       Ok(Self { stream })
//     }
// }

fn main() {
    loop {
        match run() {
            Err(err) => println!("Error: {:?}", err),
            Ok(_) => println!("Done"),
        }

        println!("Sleeping 5 seconds...");
        thread::sleep(Duration::from_secs(5));
    }

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

fn run() -> Res<()> {
    loop {
        println!("Connecting...");
        let cfg = Config::load()?;
        let mut ftp = FtpStream::connect(cfg.ip)?;
        ftp.login(cfg.user.as_str(), cfg.password.as_str())?;
        ftp.cwd(cfg.src.as_str())?;
        fs::create_dir_all(cfg.dst)?;

        println!("Retrieving listing...");
        // List all files in the FTP source directory
        let list = ftp.list(None)?;
        let lines = list.join("\n");
        println!("{}", lines.as_str());

        println!("Sleeping...");
        thread::sleep(Duration::from_secs(1));
    }
}
