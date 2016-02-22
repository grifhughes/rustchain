#![feature (libc, custom_derive, plugin)]
#![plugin(serde_macros)]
extern crate libc;
extern crate hyper;
extern crate bincode;
extern crate serde;
extern crate serde_json;

pub mod wallet;
mod conversions;
mod command_line;

use std::{io, thread, fs, path};
use std::io::Read;
use hyper::Client;
use hyper::header::Connection;
use serde_json::Value;
use std::time::Duration;
use libc::{kill, SIGTERM};

fn get_json_from_url(url: &str, cl: &Client) -> String {
    let mut json_response = String::new(); 
    let mut res = cl.get(url).header(Connection::close()).send().unwrap();   
    res.read_to_string(&mut json_response).unwrap();

    json_response
}

fn main() {
    let child = command_line::initialize_server().unwrap_or_else(|e| { panic!("Failed to start server: {}", e); });
    let client = Client::new();
    let input = io::stdin();
    
    thread::park_timeout(Duration::from_millis(2000));

    if path::Path::new("wallet.bin").exists() {        
        let mut reader = io::BufReader::new(fs::File::open("wallet.bin").unwrap());
        let wallet: wallet::Wallet = bincode::serde::deserialize_from(&mut reader, bincode::SizeLimit::Infinite).unwrap();
        
        if get_json_from_url(&wallet.login(), &client).contains("true") {
            println!("Successful login...");
            
            println!("1 - send payment\n2 - fetch wallet balance\n3 - fetch balance at specific address\n4 - list wallet addresses\n5 - generate new address\n6 - generate new address with specified label\n7 - archive address\n8 - unarchive address");

            let mut option = String::new();
            input.read_line(&mut option).expect("Failed to read");
            
            match option.trim().parse::<u8>().unwrap() {
                1 => {
                    println!("Enter destination address:");
                    let mut destination = String::new();
                    input.read_line(&mut destination).expect("Failed to read");
                    
                    println!("Enter amount in BTC:");
                    let mut amount_btc = String::new();
                    input.read_line(&mut amount_btc).expect("Failed to read");
                    let amount_satoshi = conversions::btc_to_satoshi(amount_btc
                                                                     .trim()                                                                  
                                                                     .parse::<f64>()
                                                                     .expect("Failed to parse"));

                    if amount_satoshi < wallet.available_satoshi {
                        get_json_from_url(&wallet.send_payment(&destination.trim(), amount_satoshi), &client);
                        println!("Success, pushing payment...");
                        thread::park_timeout(Duration::from_millis(5000));
                    } else {
                        println!("Error: insufficient funds");
                    }
                },
                2 => println!("{} BTC", conversions::satoshi_to_btc(wallet.available_satoshi).to_string()),
                3 => {
                    println!("Enter addresss:");
                    let mut address = String::new();
                    input.read_line(&mut address).expect("Failed to read");

                    get_json_from_url(&wallet.address_balance(&address.trim()), &client);
                },
                4 => println!("{}", get_json_from_url(&wallet.address_list(), &client)),
                5 => {
                    println!("Generating new address...");
                    get_json_from_url(&wallet.generate_address(), &client);
                    
                    thread::park_timeout(Duration::from_millis(2000));
                },
                6 => {
                    println!("Enter desired label:");
                    let mut label = String::new();
                    input.read_line(&mut label).expect("Failed to read");
                    println!("Generating new address with label {}...", label);
                    get_json_from_url(&wallet.generate_address_with_label(&label.trim()), &client);
                    
                    thread::park_timeout(Duration::from_millis(2000));
                },
                7 => {
                    println!("Enter address to archive:");
                    let mut address = String::new();
                    input.read_line(&mut address).expect("Failed to read");
                    get_json_from_url(&wallet.archive_address(&address.trim()), &client);
                    
                    thread::park_timeout(Duration::from_millis(2000));
                },
                8 => {
                    println!("Enter address to unarchive:");
                    let mut address = String::new();
                    input.read_line(&mut address).expect("Failed to read");
                    get_json_from_url(&wallet.unarchive_address(&address.trim()), &client);
                    
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
        first_wallet.guid = guid
            .trim()
            .to_string();
        
        println!("Enter blockchain wallet password: ");
        let mut password = String::new();
        input.read_line(&mut password).expect("Failed to read");
        first_wallet.main_password = password
            .trim()
            .to_string();

        println!("Logging in...");
        if get_json_from_url(&first_wallet.login(), &client).contains("true") {
            println!("Success...");
            println!("Pulling data...");

            let data: Value = serde_json::from_str(&get_json_from_url(&first_wallet.wallet_balance(), &client)).unwrap();
            let obj = data.as_object().unwrap();
            let foo = obj.get("balance").unwrap();
            
            first_wallet.available_satoshi = foo.as_f64().unwrap();
            
            let mut writer = io::BufWriter::new(fs::File::create("wallet.bin").unwrap());
            bincode::serde::serialize_into(&mut writer, &first_wallet, bincode::SizeLimit::Infinite).unwrap();
            println!("Initial wallet configured, exiting...");
        } else {
            println!("Login failed, exiting...");
        }        
    }
    
    unsafe {
        kill(child.id() as i32, SIGTERM);
    }
}        
