use ring::hmac;
use std::env;
use std::time::Duration;
use std::time::SystemTime;
use ureq;

#[derive(Debug)]
struct PublicApi {
    end_point: String,
    agent: ureq::Agent,
}

impl PublicApi {
    pub fn new(end_point: String) -> Self {
        let agent: ureq::Agent = ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .timeout_write(Duration::from_secs(5))
            .build();
        Self { end_point, agent }
    }

    pub fn get_ticker(self, pair: &str) -> Result<ureq::serde_json::Value, ureq::Error> {
        let path = format!("{}/{}/ticker", self.end_point, pair);
        let json: ureq::serde_json::Value = self.agent.get(&path).call()?.into_json()?;
        Ok(json)
    }
}

enum Side {
    Buy,
    Sell,
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::Buy => "buy",
            Self::Sell => "sell",
        };
        write!(f, "{}", s)
    }
}

enum OrderType {
    Limit,
    Market,
    Stop,
    StopLimit,
}

impl std::fmt::Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::Limit => "limit",
            Self::Market => "market",
            Self::Stop => "stop",
            Self::StopLimit => "stop_limit",
        };
        write!(f, "{}", s)
    }
}

fn sign(secret: &str, data: &str) -> hmac::Tag {
    let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
    let tag = hmac::sign(&key, data.as_bytes());
    //println!("{:?}", tag);
    tag
}

struct PrivateApi {
    end_point: String,
    agent: ureq::Agent,
    api_key: String,
    api_secret: String,
}

impl PrivateApi {
    pub fn new(end_point: String, api_key: String, api_secret: String) -> Self {
        let agent: ureq::Agent = ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .timeout_write(Duration::from_secs(5))
            .build();
        Self {
            end_point,
            agent,
            api_key,
            api_secret,
        }
    }

    fn post_query(
        self,
        path: &str,
        query: ureq::serde_json::Value,
    ) -> Result<ureq::serde_json::Value, std::io::Error> {
        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            * 1000;

        let mut message = nonce.to_string();
        message += &query.to_string();

        let s = sign(&self.api_secret, &message);
        let s: String = s
            .as_ref()
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect();
        //println!("{:?}", s);

        let req = self
            .agent
            .post(path)
            .set("Content-Type", "application/json")
            .set("ACCESS-KEY", &self.api_key)
            .set("ACCESS-NONCE", &nonce.to_string())
            .set("ACCESS-SIGNATURE", &s);
        println!("{:?}", req);

        let response = req.send_json(query).unwrap();
        println!("{:?}", response);
        response.into_json()
    }

    pub fn order_type_market(self, pair: &str, amount: f64, side: Side) {
        let path = format!("{}/user/spot/order", self.end_point);
        let params = ureq::json!({
            "pair" : pair,
            "amount": amount.to_string(),
            "side": side.to_string(),
            "type": OrderType::Market.to_string(),
            //"price": "0",
            //"post_only": "0",
            //"trigger_price": "0",
        });
        //println!("params {:?}", params);

        let res = self.post_query(&path, params).unwrap();
        println!("{:?}", res);
    }
}

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
    //let pri_end_point = "http://localhost:8080/v1";
    let pri_end_point = "https://api.bitbank.cc/v1";

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

    let pri_api = PrivateApi::new(pri_end_point.to_string(), api_key, api_secret);
    pri_api.order_type_market("btc_jpy", 0.0001, Side::Buy);
}
