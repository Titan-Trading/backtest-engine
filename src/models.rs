

pub enum OrderStatus {
    Submitted,
    PartiallyFilled,
    Filled,
    Invalid,
}

pub enum OrderSide {
    Buy,
    Sell
}

pub enum OrderType {
    Limit,
    Market
}

pub struct Order {
    pub exchange: String,
    pub symbol: String,
    pub id: i32,
    pub label: Option<String>,
    pub status: OrderStatus,
    pub side: OrderSide,
    pub fill_type: OrderType,
    pub quantity: f64,
    pub limit: Option<f64>,
    pub price: f64,
}


pub struct Metric<T> {
    pub name: String,
    pub value: T
}