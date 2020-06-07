use reqwest::Error;
use serde::{ Deserialize };
use crate::seller::{Seller, get_seller};

pub struct Item {
    pub id: String,
    pub seller: Option<Seller>,
}

#[derive(Deserialize)]
struct ItemResponse {
    id: String,
    seller_id: int,
}

pub async fn get_item(id: &str) -> Item {
    get_item_api(id).await.map_or(Item { id:String::from(""), seller: None }, |item_response| {
        let seller_model = get_seller(item_response.seller_id);

        Item {
            id: item_response.id,
            seller: seller_model,
        }
    })
}

async fn get_item_api(id: &str) -> Result<ItemResponse, Error> {
    let url = format!("https://api.mercadolibre.com/items/{}", id);

    Ok(reqwest::get(url.as_str()).await?.json().await?)
}