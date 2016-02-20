#![feature (libc, convert)]
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

fn query_url(url: String, cl: &Client) -> String {
    let mut res = cl.get(&url).header(Connection::close()).send().unwrap();
    let mut json_response = String::new();   
    res.read_to_string(&mut json_response).unwrap();

    json_response
}

fn main() {
    let child = command_line::initialize_server().unwrap_or_else(|e| { panic!("Failed to start server: {}", e); });

    let client = Client::new();
    let input = io::stdin();
    
    let mut user_wallet: wallet::Wallet = Default::default();
    
    thread::sleep(Duration::from_millis(1200));

    println!("Login to wallet\nEnter wallet identifier:");
    let mut guid = String::new();
    input.read_line(&mut guid).expect("Failed to read");
    user_wallet.guid = guid.trim().to_string();
    
    println!("Enter wallet password: ");
    let mut password = String::new();
    input.read_line(&mut password).expect("Failed to read");
    user_wallet.main_password = password.trim().to_string();
       
    if query_url(user_wallet.login(), &client).as_str().contains("true") {
        println!("Successful login...");

        println!("1 - send payment\n2 - fetch wallet balance\n3 - fetch balance at specific address\n4 - list wallet addresses");

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
                
                query_url(user_wallet.send_payment(&destination.trim(), conversions::btc_to_satoshi(amount_btc.trim().parse::<f32>().expect("Failed to parse"))), &client);
            },
            2 => println!("{}", query_url(user_wallet.wallet_balance(), &client)),
            3 => {
                println!("Enter specific addresss:");
                let mut address = String::new();
                input.read_line(&mut address).expect("Failed to read");

                query_url(user_wallet.address_balance(&address.trim()), &client);
            },
            4 => println!("{}", query_url(user_wallet.address_list(), &client)),
            _ => panic!("Error invalid option")
        }
    } else {
        println!("Invalid login, exiting...");
    }
    
    unsafe {
        kill(child.id() as i32, SIGTERM);
    }    
}
