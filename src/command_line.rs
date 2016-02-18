use std::process::Command;
use std::process::Child;
use std::io;

pub fn initialize_server() -> Result<Child, io::Error> {
    println!("Starting server...");
    Command::new("/usr/local/bin/blockchain-wallet-service").arg("start").spawn()
    
}
