use reqwest::Error;
use serde::{ Deserialize };

#[derive(Deserialize)]
struct SearchBackendResponse {
    pub results: Option<Vec<ResultsResponse>>,
    pub result_ids: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct ResultsResponse {
    pub id: String,
    pub permalink: String,
}

pub fn get_host_by_site(site: &str) -> String {
    format!("https://api.mercadolibre.com/sites/{}/search", site)
}

pub async fn get_items_ids(site: &str, params: &str) -> Vec<ResultsResponse> {
    get_search(site, params).await.map_or(Vec::new(), |search_response| {
        let mut results = search_response.results.map_or(Vec::new(), |results| results);
        let result_ids = search_response.result_ids.map_or(Vec::new(), |results| results);

        if results.is_empty() && !result_ids.is_empty() {
            for result_id in result_ids {
                results.push(ResultsResponse {
                    id: result_id.as_str().to_string(),
                    permalink: get_fallback_item_url(result_id.as_str()),
                })
            }
        }

        results
    })
}

async fn get_search(site: &str, params: &str) -> Result<SearchBackendResponse, Error> {
    let url = format!("https://api.mercadolibre.com/sites/{}/searchbackend?itemsOnly=true&{}", site, params);
    
    Ok(reqwest::get(url.as_str()).await?.json().await?)
}

fn get_fallback_item_url(item_id: &str) -> String {
    format!("https://articulo.mercadolibre.com.ar/{}-{}-_jm", &item_id[..3], &item_id[3..])
}

