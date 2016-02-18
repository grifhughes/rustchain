#![feature (libc)]
extern crate libc;
extern crate hyper;

pub mod wallet;
mod conversions;
mod command_line;

use std::io;
use std::thread;
use std::io::Read;
use hyper::Client;
use hyper::header::Connection;
use std::time::Duration;
use libc::{kill, SIGTERM};

fn main() {
    let child = command_line::initialize_server().unwrap_or_else(|e| { panic!("Failed to start server: {}", e); });
    
    let mut user_wallet: wallet::Wallet = Default::default();
    
    thread::sleep(Duration::from_millis(1200));
    
    println!("Enter wallet identifier: ");  
    io::stdin().read_line(&mut user_wallet.guid).expect("Failed to read");
    
    println!("Enter wallet password: ");
    io::stdin().read_line(&mut user_wallet.main_password).expect("Failed to read");
    
    println!("Enter destination address: ");
    let mut dest_addr = String::new();
    io::stdin().read_line(&mut dest_addr).expect("Failed to read");
        
    println!("Enter amount: (BTC)");
    let mut amount_satoshi = String::new();
    io::stdin().read_line(&mut amount_satoshi).expect("Failed to read");
    let amount_btc = conversions::satoshi_to_btc(amount_satoshi.trim().parse::<f32>().expect("Error: failed to parse")).to_string();
   
    let f1 = "http://localhost:3000/merchant/".to_string();
    let f2 = "/payment?password=";
    let f3 = "&to=";
    let f4 = "&amount=";
    let url = f1 + &user_wallet.guid + f2 + &user_wallet.main_password + f3 + &dest_addr + f4 + &amount_btc;
    println!("{}", url);

    let client = Client::new();
    
    let mut res = client.get(&url).header(Connection::close()).send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Response: {}", body);

    unsafe {
        kill(child.id() as i32, SIGTERM);
    }    
}
