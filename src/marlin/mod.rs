use crate::search::{get_host_by_site, ResultsResponse};
use crate::search::get_items_ids;
use crate::mrray::get_params;

pub struct Nemo {
    pub search_url: String,
    pub items: Vec<Item>,
}

pub struct Item {
    pub id: String,
    pub permalink: String,
}

pub async fn find_nemo(site_param: Option<&String>, mp: Option<&String>, me: Option<&String>, it: Option<&String>, variations: Option<&String>) -> Nemo {
    let site = site_param.map_or(String::from("NONE"), |s| s.to_string());
    let item_type = it.map_or(String::from("NONE"), |s| s.to_string());
    let mercado_pago = mp.map_or(String::from("NONE"), |s| s.to_string());
    let mercado_envios = me.map_or(String::from("NONE"), |s| s.to_string());
    let variations = variations.map_or(String::from("NONE"), |s| s.to_string());

    let params = get_params(site.as_str(), item_type.as_str(), mercado_pago.as_str(), mercado_envios.as_str(), variations.as_str());

    let results = get_items_ids(site.as_str(), params.as_str()).await;

    let search_url = format!("{}?{}", get_host_by_site(site.as_str()), params);

    return map_response_to_nemo(search_url.as_str(), results);
}

fn map_response_to_nemo(search_url: &str, results: Vec<ResultsResponse>) -> Nemo {
    let mut items = Vec::new();

    for result in results {
        items.push(Item {
            id: result.id,
            permalink: result.permalink
        });
    }

    return Nemo {
        search_url: search_url.to_string(),
        items
    }
}