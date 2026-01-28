use leptos::prelude::ServerFnError;
use leptos::server;
use std::sync::OnceLock;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;
use crate::backend::model::{AssetWithPrice, Category, Wallet};



pub static DB: OnceLock<Surreal<Client>> = OnceLock::new();

// Initialize once at startup
pub async fn init_db() -> Result<(), ServerFnError> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    // Sign in with credentials
    db.signin(surrealdb::opt::auth::Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("eric").use_db("Trading").await?;
    DB.set(db).expect("Failed to set database");

    Ok(())
}

// Use anywhere in your app
pub fn get_db() -> &'static Surreal<Client> {
    DB.get().expect("Database not initialized")
}

#[server(GetCategories, "/api")]
pub async fn get_categories() -> Result<Vec<Category>, ServerFnError> {
    let db = get_db();

    // Use FETCH to get the related ratings data
 let categories = db.select("category").await?;

    Ok(categories)
}

#[server(GetWallet, "/api")]
pub async fn get_wallet() -> Result<Vec<Wallet>, ServerFnError> {
    let db = get_db();
   
 let categories = db.select("wallet").await?;

    Ok(categories)
}


#[server(GetAssets, "/api")]
pub async fn get_assets() -> Result<Vec<AssetWithPrice>, ServerFnError> {
    let db = get_db();

    let mut response = db
        .query(
            "SELECT *,
                (SELECT * FROM price WHERE asset = $parent.id ORDER BY price_date DESC LIMIT 2)
                AS recent_prices
             FROM asset FETCH risk",
        )
        .await?;
    let mut assets: Vec<AssetWithPrice> = response.take(0)?;
    for item in assets.iter_mut() {
        let last_price = item.recent_prices.get(0).map(|p| p.price);
        let price_change_pct = match (item.recent_prices.get(0), item.recent_prices.get(1)) {
            (Some(latest), Some(prev)) if prev.price != 0.0 => {
                Some((latest.price - prev.price) / prev.price * 100.0)
            }
            _ => None,
        };
        item.last_price = last_price;
        item.price_change_pct = price_change_pct;
    }

    Ok(assets)
}
