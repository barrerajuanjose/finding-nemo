use reqwest::Error;
use serde::{ Deserialize };

pub struct Item {
    pub id: String,
    pub permalink: String,
    pub seller_id: u32,
    pub has_puis: bool,
    pub has_manufacturing_time: bool,
}

#[derive(Deserialize)]
struct ItemResponse {
    id: String,
    seller_id: u32,
    permalink: String,
    sale_terms: Vec<SaleTermsResponse>,
    shipping: ShippingResponse,
}

#[derive(Deserialize)]
struct SaleTermsResponse {
    id: String,
}

impl SaleTermsResponse {
    fn is_puis(&self) -> bool {
        self.id == "MANUFACTURING_TIME"
    }
}

#[derive(Deserialize)]
struct ShippingResponse {
    store_pick_up: bool,
}

impl ShippingResponse {
    fn has_puis(&self) -> bool {
        self.store_pick_up
    }
}

pub async fn get_item(id: &str) -> Item {
    get_item_api(id)
        .await
        .map_or(Item { id:String::from(""), permalink: String::from(""), seller_id: 0, has_puis: false, has_manufacturing_time: false },
                |item_response| {
                    Item {
                        id: item_response.id,
                        permalink: item_response.permalink,
                        seller_id: item_response.seller_id,
                        has_puis: item_response.shipping.has_puis(),
                        has_manufacturing_time: item_response.sale_terms.iter().find(|sale_term| sale_term.is_puis()).is_some(),
                    }
                })
}

async fn get_item_api(id: &str) -> Result<ItemResponse, Error> {
    let url = format!("https://api.mercadolibre.com/items/{}", id);
    println!("DO api call [{}]", url);
    Ok(reqwest::get(url.as_str()).await?.json().await?)
}