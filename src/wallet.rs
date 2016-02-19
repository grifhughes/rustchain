use conversions;

#[derive(Default)]
pub struct Wallet {
    pub guid: String,
    pub main_password: String,
    //TODO
    pub two_factor_auth: char,
    pub secondary_password: Option<String>   
}

impl Wallet {
    pub fn login(&self) -> String {
        "http://localhost:3000/merchant/".to_string() + &self.guid + "/login?password=" + &self.main_password + "&api_code=581dfe1f-34fc-4660-abe7-c2d0f104a546"
    }

    pub fn send_payment(&self, dest_addr: &str, amount_btc: f32) -> String {
        let amount_satoshi_string = conversions::btc_to_satoshi(amount_btc).to_string();
        "http://localhost:3000/merchant/".to_string() + &self.guid + "/payment?password=" + &self.main_password + "&to=" + dest_addr + "&amount=" + &amount_satoshi_string
    }
}


