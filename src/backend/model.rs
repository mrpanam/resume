use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Option<RecordId>,
    pub name: String,    
    pub description: String  
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub id: Option<RecordId>,
    pub amount: i64,    
    pub ccy: String ,
    pub status: String,
    pub tx_date: Datetime,
    pub note: String

}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Risk {
    pub id: Option<RecordId>,
    pub name: String,    
    pub risk_score: u8  
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    pub id: Option<RecordId>,
    pub asset: RecordId,
    pub price: f64,
    pub price_date: Datetime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: Option<RecordId>,
    pub symbol: String,
    pub category: RecordId,
    pub risk: Risk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetWithPrice {
    pub id: Option<RecordId>,
    pub symbol: String,
    pub category: RecordId,
    pub risk: Risk,
    pub recent_prices: Vec<Price>,
    pub last_price: Option<f64>,
    pub price_change_pct: Option<f64>,
}
