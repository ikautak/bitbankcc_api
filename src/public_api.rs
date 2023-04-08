use anyhow::{anyhow, Result};
use std::time::Duration;
use ureq;

#[derive(Debug)]
pub struct TickerResponse {
    pub sell: f64,
    pub buy: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub last: f64,
    pub vol: f64,
    pub timestamp: u64,
}

impl Into<TickerResponse> for ureq::serde_json::Value {
    fn into(self) -> TickerResponse {
        TickerResponse {
            sell: (self["sell"].as_str().unwrap().parse::<f64>().unwrap()),
            buy: (self["buy"].as_str().unwrap().parse::<f64>().unwrap()),
            high: (self["high"].as_str().unwrap().parse::<f64>().unwrap()),
            low: (self["low"].as_str().unwrap().parse::<f64>().unwrap()),
            open: (self["open"].as_str().unwrap().parse::<f64>().unwrap()),
            last: (self["last"].as_str().unwrap().parse::<f64>().unwrap()),
            vol: (self["vol"].as_str().unwrap().parse::<f64>().unwrap()),
            timestamp: (self["timestamp"].as_u64().unwrap()),
        }
    }
}

#[derive(Debug)]
pub struct PublicApi {
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

    pub fn get_ticker(self, pair: &str) -> Result<TickerResponse> {
        let path = format!("{}/{}/ticker", self.end_point, pair);
        let json: ureq::serde_json::Value = self.agent.get(&path).call()?.into_json()?;

        if json["success"].as_i64().unwrap() != 1 {
            return Err(anyhow!("api error {}", json["data"]["code"]));
        }

        Ok(json["data"].to_owned().into())
    }

    //pub fn get_tickers(self, ) ->
    //pub fn get_tickers_jpy(self, ) ->
    //pub fn get_depth(self, ) ->
    //pub fn get_transactions(self, ) ->
    //pub fn get_candlestick(self, ) ->
}
