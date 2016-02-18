extern crate hyper;

use std::io;
use std::io::Read;
use hyper::Client;
use hyper::header::Connection;

fn main() {
    println!("Enter wallet identifier: ");  
    let mut wallet_id = String::new();
    io::stdin().read_line(&mut wallet_id).expect("Failed to read");
    
    println!("Enter wallet password: ");
    let mut wallet_password = String::new();
    io::stdin().read_line(&mut wallet_password).expect("Failed to read");
    
    println!("Enter destination address: ");
    let mut dest_addr = String::new();
    io::stdin().read_line(&mut dest_addr).expect("Failed to read");
        
    println!("Enter amount: (BTC)");
    let mut amount_satoshi = String::new();
    io::stdin().read_line(&mut amount_satoshi).expect("Failed to read");
    let amount_btc = (amount_satoshi.trim().parse::<f32>().expect("Failed to parse") * 100000000.0).to_string();
   
    let f1 = "http://localhost:3000/merchant/".to_string();
    let f2 = "/payment?password=";
    let f3 = "&to=";
    let f4 = "&amount=";
    let url = f1 + &wallet_id + f2 + &wallet_password + f3 + &dest_addr + f4 + &amount_btc;

    let mut client = Client::new();
    let mut res = client.get(&url).header(Connection::close()).send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Response: {}", body);

    
}
