use reqwest::Error;
use serde::{ Deserialize };

pub struct Seller {
    pub id: u32,
    pub reputation: String,
}

#[derive(Deserialize)]
struct UserResponse {
    id: u32,
    seller_reputation: SellerReputation,
}

#[derive(Deserialize)]
struct SellerReputation {
    level_id: Option<String>,
    power_seller_status: Option<String>,
}

pub async fn get_seller(id: u32) -> Seller {
    get_seller_api(id).await.map_or(Seller { id:0, reputation:String::from("") }, |seller_response| {
        let level_id = seller_response.seller_reputation.level_id.map_or(String::from("NEWBIE"), |s| s);
        let power_seller_status = seller_response.seller_reputation.power_seller_status.map_or(String::from(""), |s| s);

        let reputation = if power_seller_status == "" {
            level_id
        } else {
            format!("{}_{}", level_id, power_seller_status)
        };

        Seller {
            id: seller_response.id,
            reputation
        }
    })
}

async fn get_seller_api(id: u32) -> Result<UserResponse, Error> {
    let url = format!("https://api.mercadolibre.com/users/{}", id);
    println!("DO api call [{}]", url);
    Ok(reqwest::get(url.as_str()).await?.json().await?)
}