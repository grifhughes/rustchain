use conversions;

const API_CODE: &'static str = "581dfe1f-34fc-4660-abe7-c2d0f104a546";
const MERCHANT_ENDPOINT: &'static str = "http://localhost:3000/merchant/";
const NEW_WALLET_ENDPOINT: &'static str = "http://localhost:3000/v2/create";

#[derive(Serialize, Deserialize, Default)]
pub struct Wallet {
    pub guid: String,
    pub main_password: String,
    //TODO
    pub two_factor_auth: char,
    pub secondary_password: Option<String>   
}

impl Wallet {
    pub fn login(&self) -> String {
        MERCHANT_ENDPOINT.to_string() + &self.guid + "/login?password=" + &self.main_password + "&api_code=" + API_CODE
    }

    //TODO: Implement in main
    pub fn create_new(&self) -> String {
        NEW_WALLET_ENDPOINT.to_string() + "?password=" + &self.main_password + "&api_code=" + API_CODE
    }
    
    pub fn send_payment(&self, dest_addr: &str, amount_btc: f32) -> String {
        let amount_satoshi_string = conversions::btc_to_satoshi(amount_btc).to_string();
        MERCHANT_ENDPOINT.to_string() + &self.guid + "/payment?password=" + &self.main_password + "&to=" + dest_addr + "&amount=" + &amount_satoshi_string
    }

    pub fn send_payment_from_addr(&self, dest_addr: &str, amount_btc: f32, from_addr: &str) -> String {
        let amount_satoshi_string = conversions::btc_to_satoshi(amount_btc).to_string();
        MERCHANT_ENDPOINT.to_string() + &self.guid + "/payment?password=" + &self.main_password + "&to=" + dest_addr + "&amount=" + &amount_satoshi_string + "&from=" + from_addr
    }

    pub fn send_payment_from_addr_with_note(&self, dest_addr: &str, amount_btc: f32, from_addr: &str, note: &str) -> String {
        let amount_satoshi_string = conversions::btc_to_satoshi(amount_btc).to_string();
        MERCHANT_ENDPOINT.to_string() + &self.guid + "/payment?password=" + &self.main_password + "&to=" + dest_addr + "&amount=" + &amount_satoshi_string + "&from=" + from_addr + "&note=" + note
    }

    pub fn wallet_balance(&self) -> String {
        MERCHANT_ENDPOINT.to_string() + &self.guid + "/balance?password=" + &self.main_password
    }

    pub fn address_list(&self) -> String {
        MERCHANT_ENDPOINT.to_string() + &self.guid + "/list?password=" + &self.main_password
    }

    pub fn address_balance(&self, addr: &str) -> String {
        MERCHANT_ENDPOINT.to_string() + &self.guid + "/address_balance?address=" + addr + "&password=" + &self.main_password
    }

    pub fn generate_address(&self) -> String {
        MERCHANT_ENDPOINT.to_string() + &self.guid + "/new_address?password=" + &self.main_password
    }

    pub fn generate_address_with_label(&self, label: &str) -> String {
        MERCHANT_ENDPOINT.to_string() + &self.guid + "/new_address?password=" + &self.main_password + "&label=" + label
    }  
}
