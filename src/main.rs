use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::thread;
use std::time::Duration;
use std::{error::Error, fs};

use confy::{self};
use ftp::{FtpError, FtpStream};
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
            src: "/home/ftp/".to_string(),
            dst: "c:\\temp\\photos".to_string(),
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

fn download(ftp: &mut FtpStream, src_name: &str, dst_dir: &str) -> Res<()> {
    let _ = ftp.retr(src_name, |stream| {
        let mut buf = Vec::new();
        stream
            .read_to_end(&mut buf)
            .and_then(|_| {
                let src_path = Path::new(src_name);
                let filename = src_path.file_name().unwrap();
                let dir_path = Path::new(dst_dir);
                let dst_path = dir_path.join(filename);
                println!("Creating file {:?}", dst_path);
                File::create(dst_path).and_then(|mut file| file.write(&buf))
            })
            .map_err(|e| FtpError::ConnectionError(e))
    })?;

    ftp.rm(src_name)?;

    Ok(())
}

fn run() -> Res<()> {
    loop {
        println!("Connecting...");
        let cfg = Config::load()?;
        let mut ftp = FtpStream::connect(cfg.ip)?;
        let tcp = ftp.get_ref();
        tcp.set_read_timeout(Some(Duration::from_secs(5)))?;

        ftp.login(cfg.user.as_str(), cfg.password.as_str())?;

        let dst = cfg.dst.as_str();
        fs::create_dir_all(dst)?;

        println!("Retrieving listing...");
        // List all files in the FTP source directory
        let list = ftp.nlst(Some(cfg.src.as_str()))?;
        // let lines = list.join("\n");
        // println!("{}", lines.as_str());

        println!("Retrieving time stamps...");

        let pairs: Vec<_> = list
            .iter()
            .flat_map(|path| match ftp.mdtm(path) {
                Ok(otm) => otm.map(|date| (path, date)),
                Err(_) => None,
            })
            .collect();

        println!("Sleeping 5 seconds...");
        thread::sleep(Duration::from_secs(5));

        println!("Finding downloadable files...");

        let ready: Vec<_> = pairs
            .iter()
            .flat_map(|(path, date)| match ftp.mdtm(path) {
                Ok(otm) => otm.map(|mt| (*path, mt.eq(date))),
                Err(_) => None,
            })
            .flat_map(|(path, stable)| if stable { Some(path) } else { None })
            .collect();

        for &path in ready.iter() {
          println!("Downloading {}...", path);
          download(&mut ftp, &path, cfg.dst.as_str())?;
        }
    }
}
