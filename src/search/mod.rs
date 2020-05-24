use reqwest::Error;
use std::collections::HashMap;

pub fn get_host_by_site(site: String) -> String {
    format!("https://api.mercadolibre.com/sites/{}/search", site)
}

pub async fn get_items_ids(url: &str) -> Result<String, Error> {
    let resp = reqwest::get(url).await?
        .json::<HashMap<String, String>>()
        .await?;

    println!("success! {:#?}", resp);

    /*if resp.status().is_success() {
        println!("success! {}", resp.json());
    } else if resp.status().is_server_error() {
        println!("server error!");
    } else {
        println!("Something else happened. Status: {:?} - {}", resp.status(), url);
    }*/

    Ok(String::from(url))
}