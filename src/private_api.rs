use crate::common::{OrderSide, OrderType};
use ring::hmac;
use std::time::Duration;
use std::time::SystemTime;
use ureq;

fn sign(secret: &str, data: &str) -> hmac::Tag {
    let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
    let tag = hmac::sign(&key, data.as_bytes());
    //println!("{:?}", tag);
    tag
}

#[derive(Debug)]
struct PrivateApi {
    end_point: String,
    agent: ureq::Agent,
    api_key: String,
    api_secret: String,
}

impl PrivateApi {
    pub fn new(end_point: &str, api_key: String, api_secret: String) -> Self {
        let agent: ureq::Agent = ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .timeout_write(Duration::from_secs(5))
            .build();
        Self {
            end_point: end_point.to_string(),
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
            .as_millis()
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

    pub fn order_type_market(self, pair: &str, amount: f64, side: OrderSide) {
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
