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
