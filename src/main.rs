#![feature (libc, custom_derive, plugin)]
#![plugin(serde_macros)]
extern crate libc;
extern crate hyper;
extern crate bincode;
extern crate serde;

pub mod wallet;
mod conversions;
mod command_line;

use std::{io, thread, fs, path};
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
    let mut child = command_line::initialize_server().unwrap_or_else(|e| { panic!("Failed to start server: {}", e); });

    let input = io::stdin();
    
    thread::sleep(Duration::from_millis(1200));

    if path::Path::new("wallet.bin").exists() {
        let client = Client::new();
        
        let mut reader = io::BufReader::new(fs::File::open("wallet.bin").unwrap());
        let wallet: wallet::Wallet = bincode::serde::deserialize_from(&mut reader, bincode::SizeLimit::Infinite).unwrap();

        if query_url(wallet.login(), &client).as_str().contains("true") {
            println!("Successful login...");

            println!("1 - send payment\n2 - fetch wallet balance\n3 - fetch balance at specific address\n4 - list wallet addresses\n5 - generate new address\n6 - generate new address with specified label");

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
                    
                    query_url(wallet.send_payment(&destination.trim(), conversions::btc_to_satoshi(amount_btc.trim().parse::<f32>().expect("Failed to parse"))), &client);
                },
                2 => println!("{}", query_url(wallet.wallet_balance(), &client)),
                3 => {
                    println!("Enter addresss:");
                    let mut address = String::new();
                    input.read_line(&mut address).expect("Failed to read");

                    query_url(wallet.address_balance(&address.trim()), &client);
                },
                4 => println!("{}", query_url(wallet.address_list(), &client)),
                5 => {
                    println!("Generating new address...");
                    println!("{}", query_url(wallet.generate_address(), &client));
                    thread::park_timeout(Duration::from_millis(2000));
                },
                6 => {
                    println!("Enter desired label:");
                    let mut label = String::new();
                    input.read_line(&mut label).expect("Failed to read");
                    println!("Generating new address with label {}...", label);
                    query_url(wallet.generate_address_with_label(&label), &client);
                    thread::park_timeout(Duration::from_millis(2000));
            },
                _ => panic!("Error invalid option")
            }
        } else {
            println!("Invalid login, exiting...");
        }
    } else {
        println!("Performing first time setup...");

        let mut first_wallet: wallet::Wallet = Default::default();

        println!("Enter blockchain wallet identifier:");
        let mut guid = String::new();
        input.read_line(&mut guid).expect("Failed to read");
        first_wallet.guid = guid.trim().to_string();
        
        println!("Enter blockchain wallet password: ");
        let mut password = String::new();
        input.read_line(&mut password).expect("Failed to read");
        first_wallet.main_password = password.trim().to_string();

        let mut writer = io::BufWriter::new(fs::File::create("wallet.bin").unwrap());
        bincode::serde::serialize_into(&mut writer, &first_wallet, bincode::SizeLimit::Infinite).unwrap();
        println!("Initial wallet configured, exiting...");
    }
    
    unsafe {
        kill(child.id() as i32, SIGTERM);
    }
}        
