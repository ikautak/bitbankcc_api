mod common;
mod private_api;
mod public_api;

use crate::common::{CandleType, OrderSide, OrderType};
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

    //let ticker = pub_api.get_ticker("btc_jpy").unwrap();
    //println!("{:?}", ticker);

    //let tickers = pub_api.get_tickers().unwrap();
    //println!("{:?}", tickers);

    let tickers = pub_api.get_tickers_jpy().unwrap();
    println!("{:?}", tickers);

    //let depth = pub_api.get_depth("btc_jpy").unwrap();
    //println!("{:?}", depth);

    //let transactions = pub_api.get_transactions("btc_jpy", None).unwrap();
    //println!("{:?}", transactions);

    //let candle_stick = pub_api
    //    .get_candlestick("btc_jpy", CandleType::_5min, "20230408")
    //    .unwrap();
    //println!("{:?}", candle_stick);

    //let pri_api = PrivateApi::new(pri_end_point.to_string(), api_key, api_secret);
    //pri_api.order_type_market("btc_jpy", 0.0001, OrderSide::Buy);
}
