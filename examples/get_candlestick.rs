use rust_bb::common::CandleType;
use rust_bb::public_api::PublicApi;

fn main() {
    let pub_end_point = "https://public.bitbank.cc";
    //let pub_end_point = "http://localhost:8080";
    let pub_api = PublicApi::new(pub_end_point);

    let candle_stick = pub_api
        .get_candlestick("btc_jpy", CandleType::_5min, "20230410")
        .unwrap();
    println!("{:?}", candle_stick);
}
