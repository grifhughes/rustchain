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

fn query_url(url: String, cl: &Client) {
    let mut res = cl.get(&url).header(Connection::close()).send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    println!("Response: {}", body);
}

fn main() {
    let child = command_line::initialize_server().unwrap_or_else(|e| { panic!("Failed to start server: {}", e); });

    let client = Client::new();
    let input = io::stdin();
    
    let mut user_wallet: wallet::Wallet = Default::default();
    
    thread::sleep(Duration::from_millis(1200));

    println!("Login to wallet");
    println!("Enter wallet identifier: ");  
    input.read_line(&mut user_wallet.guid).expect("Failed to read");
    
    println!("Enter wallet password: ");
    input.read_line(&mut user_wallet.main_password).expect("Failed to read");
       
    query_url(user_wallet.login(), &client);

    println!("Do you wish to send a payment?");
    let mut payment = String::new();
    input.read_line(&mut payment).expect("Failed to read");
    
    if payment.trim() == "yes" {
        println!("Enter a destination address:");
        let mut destination = String::new();
        input.read_line(&mut destination).expect("Failed to read");
        println!("Enter amount in BTC:");
        let mut amount = String::new();
        input.read_line(&mut amount).expect("Failed to read");
        
        query_url(user_wallet.send_payment(&destination, amount.trim().parse::<f32>().expect("Failed to parse")), &client);        
    } else {
        // TODO
    }
    
    unsafe {
        kill(child.id() as i32, SIGTERM);
    }    
}
