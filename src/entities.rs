use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Operation {
    operation_id: u32,
    quantity: String,
    price: String,
    fee_rate: String,
    executed_timestamp: String,
}

#[derive(Deserialize, Debug)]
pub struct Order {
    order_id: u32,
    coin_pair: String,
    order_type: u16,
    status: u16,
    has_fills: bool,
    quantity: String,
    limit_price: String,
    executed_quantity: String,
    executed_price_avg: String,
    fee: String,
    created_timestamp: String,
    updated_timestamp: String,
    operations: Vec<Operation>,
}

#[derive(Deserialize, Debug)]
pub struct ListOrderResponse {
    orders: Vec<Order>,
}

#[derive(Deserialize, Debug)]
pub struct APIResponse {
    pub response_data: ListOrderResponse,
    status_code: u16,
    server_unix_timestamp: String
}

