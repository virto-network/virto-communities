use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    data: Vec<Vec<f64>>,
    code: u16,
    msg: String,
}

pub enum Error {
    PriceNotFound,
}

impl Response {
    pub fn get_price(&self) -> f64 {
        let data = self.data.get(self.data.len() - 1).expect("Should get data");
        let price = *data.get(1).expect("Should get price");
        price
    }
}

pub enum Tokens {
    KSM,
}

impl Tokens {
    pub fn name(&self) -> &str {
        match self {
            Tokens::KSM => "kusama",
        }
    }
}
