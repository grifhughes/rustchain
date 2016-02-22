pub fn btc_to_satoshi(amount: f64) -> f64 {
    amount * 100000000.0
}

pub fn satoshi_to_btc(amount: f64) -> f64 {
    amount / 100000000.0
}

