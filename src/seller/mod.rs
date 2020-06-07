use reqwest::Error;
use serde::{ Deserialize };

struct Seller {
    id: int,
    reputation: String,
}

#[derive(Deserialize)]
struct UserResponse {
    id: int,
    seller_reputation: SellerReputation,
}

#[derive(Deserialize)]
struct SellerReputation {
    level_id: Optional<String>,
    power_seller_status: Optional<String>,
}

pub async fn get_seller(id: &int) -> Seller {
    get_seller_api(id).await.map_or(Seller { id:0, reputation:String::from("") }, |seller_response| {
        let level_id = seller_response.seller_reputation.level_id.map_or(String::from("NEWBIE", |s| s));
        let power_seller_status = seller_response.seller_reputation.power_seller_status.map_or(String::from("", |s| s));

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

async fn get_seller_api(id: &int) -> Result<UserResponse, Error> {
    let url = format!("https://api.mercadolibre.com/users/{}", id);

    Ok(reqwest::get(url.as_str()).await?.json().await?)
}