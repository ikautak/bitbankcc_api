use rust_bb::public_api::PublicApi;

fn main() {
    let pub_end_point = "https://public.bitbank.cc";
    //let pub_end_point = "http://localhost:8080";
    let pub_api = PublicApi::new(pub_end_point);

    let transactions = pub_api.get_transactions("btc_jpy", None).unwrap();
    println!("{:?}", transactions);
}
