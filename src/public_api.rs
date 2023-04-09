use anyhow::{anyhow, Result};
use std::time::Duration;
use ureq;

use crate::common::OrderSide;

#[derive(Debug)]
pub struct TickerInfo {
    pub sell: f64,
    pub buy: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub last: f64,
    pub vol: f64,
    pub timestamp: u64,
}

impl Into<TickerInfo> for ureq::serde_json::Value {
    fn into(self) -> TickerInfo {
        TickerInfo {
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
pub struct DepthInfo {
    pub asks: Vec<(f64, f64)>,
    pub bids: Vec<(f64, f64)>,
}

impl Into<DepthInfo> for ureq::serde_json::Value {
    fn into(self) -> DepthInfo {
        let asks_array = self["asks"].as_array().unwrap();
        let mut asks: Vec<(f64, f64)> = Vec::with_capacity(asks_array.len());

        for e in asks_array {
            let e = e.as_array().unwrap();
            assert!(e.len() == 2);
            let price = e[0].as_str().unwrap().parse::<f64>().unwrap();
            let amount = e[1].as_str().unwrap().parse::<f64>().unwrap();
            asks.push((price, amount));
        }

        let bids_array = self["bids"].as_array().unwrap();
        let mut bids: Vec<(f64, f64)> = Vec::with_capacity(bids_array.len());

        for e in bids_array {
            let e = e.as_array().unwrap();
            assert!(e.len() == 2);
            let price = e[0].as_str().unwrap().parse::<f64>().unwrap();
            let amount = e[1].as_str().unwrap().parse::<f64>().unwrap();
            bids.push((price, amount));
        }

        DepthInfo { asks, bids }
    }
}

#[derive(Debug)]
pub struct TransactionInfo {
    pub transaction_id: u64,
    pub side: OrderSide,
    pub price: f64,
    pub amount: f64,
    pub executed_at: u64,
}

impl Into<TransactionInfo> for ureq::serde_json::Value {
    fn into(self) -> TransactionInfo {
        let side = match self["side"].as_str().unwrap() {
            "buy" => OrderSide::Buy,
            "sell" => OrderSide::Sell,
            _ => panic!("invalid side {}", self["side"]),
        };

        TransactionInfo {
            transaction_id: self["transaction_id"].as_u64().unwrap(),
            side,
            price: self["price"].as_str().unwrap().parse::<f64>().unwrap(),
            amount: self["amount"].as_str().unwrap().parse::<f64>().unwrap(),
            executed_at: self["executed_at"].as_u64().unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Transactions {
    pub data: Vec<TransactionInfo>,
}

impl Into<Transactions> for ureq::serde_json::Value {
    fn into(self) -> Transactions {
        let tx_array = self["transactions"].as_array().unwrap();
        let mut tx: Vec<TransactionInfo> = Vec::with_capacity(tx_array.len());

        for e in tx_array {
            tx.push(e.to_owned().into());
        }

        Transactions { data: tx }
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

    pub fn get_ticker(self, pair: &str) -> Result<TickerInfo> {
        let path = format!("{}/{}/ticker", self.end_point, pair);
        let json: ureq::serde_json::Value = self.agent.get(&path).call()?.into_json()?;
        //println!("{:?}", json);

        if json["success"].as_i64().unwrap() != 1 {
            return Err(anyhow!("api error {}", json["data"]["code"]));
        }

        Ok(json["data"].to_owned().into())
    }

    pub fn get_depth(self, pair: &str) -> Result<DepthInfo> {
        let path = format!("{}/{}/depth", self.end_point, pair);
        let json: ureq::serde_json::Value = self.agent.get(&path).call()?.into_json()?;
        //println!("{:?}", json);

        if json["success"].as_i64().unwrap() != 1 {
            return Err(anyhow!("api error {}", json["data"]["code"]));
        }

        Ok(json["data"].to_owned().into())
    }

    pub fn get_transactions(self, pair: &str, yyyymmdd: Option<&str>) -> Result<Transactions> {
        let mut path = format!("{}/{}/transactions", self.end_point, pair);
        if yyyymmdd.is_some() {
            path = path + "/" + yyyymmdd.unwrap();
        }

        let json: ureq::serde_json::Value = self.agent.get(&path).call()?.into_json()?;
        println!("{:?}", json);

        if json["success"].as_i64().unwrap() != 1 {
            return Err(anyhow!("api error {}", json["data"]["code"]));
        }

        Ok(json["data"].to_owned().into())
    }

    //pub fn get_tickers(self, ) ->
    //pub fn get_tickers_jpy(self, ) ->
    //pub fn get_candlestick(self, ) ->
}
