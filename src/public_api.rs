use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::time::Duration;
use ureq;

use crate::common::{CandleType, OrderSide};

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
        // "sell" and "buy" may be Null, return NaN.
        let sell = match self["sell"].as_str() {
            Some(x) => x.parse::<f64>().unwrap(),
            None => std::f64::NAN,
        };
        let buy = match self["buy"].as_str() {
            Some(x) => x.parse::<f64>().unwrap(),
            None => std::f64::NAN,
        };

        TickerInfo {
            sell,
            buy,
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
pub struct Tickers {
    data: HashMap<String, TickerInfo>,
}

impl Into<Tickers> for ureq::serde_json::Value {
    fn into(self) -> Tickers {
        let ticker_array = self.as_array().unwrap();
        let mut ticker_map = HashMap::new();

        for e in ticker_array {
            println!("{}", e["pair"]);
        }

        for e in ticker_array {
            ticker_map.insert(e["pair"].as_str().unwrap().to_string(), e.to_owned().into());
        }

        Tickers { data: ticker_map }
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
pub struct CandleStick {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub timestamp: u64,
}

impl Into<CandleStick> for ureq::serde_json::Value {
    fn into(self) -> CandleStick {
        let ohlcv = self.as_array().unwrap();
        assert!(ohlcv.len() == 6);

        CandleStick {
            open: ohlcv[0].as_str().unwrap().parse::<f64>().unwrap(),
            high: ohlcv[1].as_str().unwrap().parse::<f64>().unwrap(),
            low: ohlcv[2].as_str().unwrap().parse::<f64>().unwrap(),
            close: ohlcv[3].as_str().unwrap().parse::<f64>().unwrap(),
            volume: ohlcv[4].as_str().unwrap().parse::<f64>().unwrap(),
            timestamp: ohlcv[5].as_u64().unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct CandleStickInfo {
    ohlcv: Vec<CandleStick>,
}

impl Into<CandleStickInfo> for ureq::serde_json::Value {
    fn into(self) -> CandleStickInfo {
        let candle_stick = self.as_array().unwrap();
        assert!(candle_stick.len() > 0);
        let ohlcv_array = candle_stick[0]["ohlcv"].as_array().unwrap();
        let mut out: Vec<CandleStick> = Vec::with_capacity(ohlcv_array.len());

        for e in ohlcv_array {
            out.push(e.to_owned().into());
        }

        CandleStickInfo { ohlcv: out }
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

    pub fn get_tickers(self) -> Result<Tickers> {
        let path = format!("{}/tickers", self.end_point);
        let json: ureq::serde_json::Value = self.agent.get(&path).call()?.into_json()?;
        println!("{:?}", json);

        if json["success"].as_i64().unwrap() != 1 {
            return Err(anyhow!("api error {}", json["data"]["code"]));
        }

        Ok(json["data"].to_owned().into())
    }

    //pub fn get_tickers_jpy(self, ) ->

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
        //println!("{:?}", json);

        if json["success"].as_i64().unwrap() != 1 {
            return Err(anyhow!("api error {}", json["data"]["code"]));
        }

        Ok(json["data"].to_owned().into())
    }

    pub fn get_candlestick(
        self,
        pair: &str,
        candle_type: CandleType,
        yyyy: &str,
    ) -> Result<CandleStickInfo> {
        match candle_type {
            CandleType::_1min
            | CandleType::_5min
            | CandleType::_15min
            | CandleType::_30min
            | CandleType::_1hour => {
                // YYYYMMDD
                assert!(yyyy.len() == 8)
            }
            // YYYY
            _ => assert!(yyyy.len() == 4),
        }

        let path = format!(
            "{}/{}/candlestick/{}/{}",
            self.end_point, pair, candle_type, yyyy
        );
        println!("{}", path);

        //let json = self.agent.get(&path).call()?.into_json()?;
        let json: ureq::serde_json::Value =
            self.agent.get(&path).call().unwrap().into_json().unwrap();
        println!("{:?}", json);

        if json["success"].as_i64().unwrap() != 1 {
            return Err(anyhow!("api error {}", json["data"]["code"]));
        }

        Ok(json["data"]["candlestick"].to_owned().into())
    }
}
