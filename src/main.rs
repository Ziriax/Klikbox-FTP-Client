use std::time::Duration;

use ftp::FtpStream;

fn main() {
    let mut ftp_stream = FtpStream::connect("127.0.0.1:21").unwrap();
    println!("Connected!");
    ftp_stream.get_ref().set_read_timeout(Some(Duration::from_secs(10)))
                .expect("set_read_timeout call failed");
    ftp_stream.login("Doe", "mumble").unwrap();
    println!("Logged in!");
    let cwd = ftp_stream.pwd().unwrap();
    println!("CD: {:?}", cwd);
    let list = ftp_stream.nlst(Some(&cwd)).unwrap();
    println!("Dir: {:?}!", &list);
    let _ = ftp_stream.quit();
}
