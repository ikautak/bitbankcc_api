mod common;
mod private_api;
mod public_api;

use crate::common::{OrderSide, OrderType};
use crate::public_api::PublicApi;
use std::env;

fn get_env() -> (String, String) {
    let api_key = env::var("BITBANK_API_KEY").unwrap();
    let api_secret = env::var("BITBANK_API_SECRET").unwrap();
    //println!("api_key {}", api_key);
    //println!("api_secret {}", api_secret);

    (api_key, api_secret)
}

fn main() {
    let (api_key, api_secret) = get_env();

    //let pub_end_point = "http://localhost:8080";
    let pub_end_point = "https://public.bitbank.cc";
    let pri_end_point = "http://localhost:8080/v1";
    //let pri_end_point = "https://api.bitbank.cc/v1";

    let pub_api = PublicApi::new(pub_end_point.to_string());
    let json = pub_api.get_ticker("btc_jpy").unwrap();
    let btc_str = json["data"]["buy"].as_str().unwrap();
    let btc_jpy = btc_str.parse::<u64>().unwrap();
    println!("btc_jpy {}", btc_jpy);
    let mut amount: f64 = 3000.0 / (btc_jpy as f64);
    amount = amount * 10000.0;
    amount = amount.floor() / 10000.0;
    println!("amount {}", amount);
    if amount < 0.0001 {
        println!("no enough jpn");
        return;
    }

    //let pri_api = PrivateApi::new(pri_end_point.to_string(), api_key, api_secret);
    //pri_api.order_type_market("btc_jpy", 0.0001, OrderSide::Buy);
}
