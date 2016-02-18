#[derive(Default)]
pub struct Wallet {
    pub guid: String,
    pub main_password: String,
    pub two_factor_auth: char,
    pub secondary_password: Option<String>   
}


