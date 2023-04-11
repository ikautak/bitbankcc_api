use bitbankcc_api::{OrderSide, PrivateApi};
use std::env;

fn get_env() -> (String, String) {
    let api_key = env::var("BITBANK_API_KEY").unwrap();
    let api_secret = env::var("BITBANK_API_SECRET").unwrap();

    (api_key, api_secret)
}

fn main() {
    let (api_key, api_secret) = get_env();

    let pri_end_point = "http://localhost:8080/v1";
    //let pri_end_point = "https://api.bitbank.cc/v1";

    let pri_api = PrivateApi::new(pri_end_point, api_key, api_secret);
    pri_api.order_type_market("btc_jpy", 0.0001, OrderSide::Buy);
}
