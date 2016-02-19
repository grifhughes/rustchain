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
    let mut json_response = String::new();   
    res.read_to_string(&mut json_response).unwrap();
   
    println!("Response: {}", json_response);
}

fn main() {
    let child = command_line::initialize_server().unwrap_or_else(|e| { panic!("Failed to start server: {}", e); });

    let client = Client::new();
    let input = io::stdin();
    
    let mut user_wallet: wallet::Wallet = Default::default();
    
    thread::sleep(Duration::from_millis(1200));

    println!("Login to wallet\nEnter wallet identifier:");
    input.read_line(&mut user_wallet.guid).expect("Failed to read");
    
    println!("Enter wallet password: ");
    input.read_line(&mut user_wallet.main_password).expect("Failed to read");
       
    query_url(user_wallet.login(), &client);

    //assumes successful login, TODO add error checking
    println!("Options\n1 - send payment\n2 - fetch wallet balance\n3 - fetch balance at specific address\n4 - list wallet addresses");

    let mut option = String::new();
    input.read_line(&mut option).expect("Failed to read");

    match option.trim().parse::<i8>().unwrap() {
        1 => {
            println!("Enter destination address:");
            let mut destination = String::new();
            input.read_line(&mut destination).expect("Failed to read");
            
            println!("Enter amount in BTC:");
            let mut amount_btc = String::new();
            input.read_line(&mut amount_btc).expect("Failed to read");
            
            query_url(user_wallet.send_payment(&destination, conversions::btc_to_satoshi(amount_btc.trim().parse::<f32>().expect("Failed to parse"))), &client);
        },
        2 => query_url(user_wallet.wallet_balance(), &client),      
        _ => panic!("Error invalid option")
    }
    
    unsafe {
        kill(child.id() as i32, SIGTERM);
    }    
}
