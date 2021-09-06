use std::error::Error;
use std::fmt::Debug;

use confy::{self};
use serde::{Deserialize, Serialize};

static CONFIG_FILE_NAME: &str = "klikbox-ftp-client";

type Res<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    ip: String,
    src: String,
    dst: String,
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
    match run() {
        Err(err) => println!("Error: {:?}", err),
        Ok(_) => println!("Done"),
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
    let _ = Config::load()?;
    Ok(())
}
