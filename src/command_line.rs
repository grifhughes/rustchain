use std::process::{Command, Child};
use std::io;

pub fn initialize_server() -> Result<Child, io::Error> {
    println!("Starting server...");
    Command::new("blockchain-wallet-service").arg("start").spawn()
}
