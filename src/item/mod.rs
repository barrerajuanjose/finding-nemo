use reqwest::Error;
use serde::{ Deserialize };

pub struct Item {
    pub id: String,
    pub permalink: String,
    pub seller_id: u32,
}

#[derive(Deserialize)]
struct ItemResponse {
    id: String,
    seller_id: u32,
    permalink: String,
}

pub async fn get_item(id: &str) -> Item {
    get_item_api(id).await.map_or(Item { id:String::from(""), permalink: String::from(""), seller_id: 0 }, |item_response| {
        Item {
            id: item_response.id,
            permalink: item_response.permalink,
            seller_id: item_response.seller_id,
        }
    })
}

async fn get_item_api(id: &str) -> Result<ItemResponse, Error> {
    let url = format!("https://api.mercadolibre.com/items/{}", id);
    println!("DO api call [{}]", url);
    Ok(reqwest::get(url.as_str()).await?.json().await?)
}