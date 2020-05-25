use reqwest::Error;
use serde::{ Deserialize };

#[derive(Deserialize)]
struct SearchBackendResponse {
    pub result_ids: Vec<String>,
}

pub fn get_host_by_site(site: String) -> String {
    format!("https://api.mercadolibre.com/sites/{}/search", site)
}

pub async fn get_items_ids(site: &str, params: &str) -> Vec<String> {
    get_search(site, params).await.map_or(Vec::new(), |search_response| search_response.result_ids)
}

async fn get_search(site: &str, params: &str) -> Result<SearchBackendResponse, Error> {
    let url = format!("https://api.mercadolibre.com/sites/{}/searchbackend{}", site, params);

    Ok(reqwest::get(url.as_str()).await?.json().await?)
}

