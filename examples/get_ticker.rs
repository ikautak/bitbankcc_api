use bitbankcc_api::PublicApi;

fn main() {
    let pub_end_point = "https://public.bitbank.cc";
    //let pub_end_point = "http://localhost:8080";
    let pub_api = PublicApi::new(pub_end_point);

    let ticker = pub_api.get_ticker("btc_jpy").unwrap();
    println!("{:?}", ticker);
}
