

#[derive(Clone)]
pub enum OrderStatus {
    Submitted,
    PartiallyFilled,
    Filled,
    Invalid,
}

#[derive(Clone)]
pub enum OrderSide {
    Buy,
    Sell
}

#[derive(Clone)]
pub enum OrderType {
    Limit,
    Market
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Metric<T> {
    pub name: String,
    pub value: T
}