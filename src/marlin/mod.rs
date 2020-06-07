use crate::search::{get_host_by_site, ResultsResponse, get_seller_search};
use crate::search::get_items_ids;
use crate::mrray::get_params;
use std::collections::{HashMap, HashSet};
use crate::item::get_item;
use crate::seller::get_seller;

pub struct Nemo {
    pub search_url: String,
    pub items: Vec<ItemNemo>,
    pub sellers_types: HashMap<String, Vec<SellerNemo>>,
}

pub struct ItemNemo {
    pub id: String,
    pub permalink: String,
}

pub struct SellerNemo {
    pub id: u32,
    pub reputation: String,
    pub search_url: String,
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

    return map_response_to_nemo(search_url.as_str(), results, site.as_str()).await;
}

async fn map_response_to_nemo(search_url: &str, results: Vec<ResultsResponse>, site: &str) -> Nemo {
    let mut items = Vec::new();
    let mut sellers_types = HashMap::new();
    let mut sellers_ids = HashSet::new();

    for result in results {
        let item = get_item(result.id.as_str()).await;

        items.push(ItemNemo {
            id: item.id,
            permalink: item.permalink
        });

        sellers_ids.insert(item.seller_id);
    }

    for seller_id in sellers_ids {
        let seller = get_seller(seller_id).await;
        let sellers_types_key = seller.reputation.as_str().to_string();

        sellers_types.entry(sellers_types_key).or_insert_with(Vec::new).push(SellerNemo {
            id: seller.id,
            reputation: seller.reputation,
            search_url: get_seller_search(site, seller.id),
        })
    }

    Nemo {
        search_url: search_url.to_string(),
        items,
        sellers_types
    }
}