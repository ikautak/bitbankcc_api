#[derive(Debug)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl std::fmt::Display for OrderSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::Buy => "buy",
            Self::Sell => "sell",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum OrderType {
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

#[derive(Debug)]
pub enum CandleType {
    _1min,
    _5min,
    _15min,
    _30min,
    _1hour,
    _4hour,
    _8hour,
    _12hour,
    _1day,
    _1week,
    _1month,
}

impl std::fmt::Display for CandleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::_1min => "1min",
            Self::_5min => "5min",
            Self::_15min => "15min",
            Self::_30min => "30min",
            Self::_1hour => "1hour",
            Self::_4hour => "4hour",
            Self::_8hour => "8hour",
            Self::_12hour => "12hour",
            Self::_1day => "1day",
            Self::_1week => "1week",
            Self::_1month => "1month",
        };
        write!(f, "{}", s)
    }
}
