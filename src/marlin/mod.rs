use crate::search::{get_host_by_site, ResultsResponse, get_seller_search};
use crate::search::get_items_ids;
use crate::mrray::{get_params, filter_item_nemo};
use std::collections::{HashMap, HashSet};
use crate::item::get_item;
use crate::seller::get_seller;
use futures::future;

pub struct Nemo {
    pub search_url: String,
    pub items: Vec<ItemNemo>,
    pub sellers_types: HashMap<String, Vec<SellerNemo>>,
}

pub struct ItemNemo {
    pub id: String,
    pub permalink: String,
    pub has_puis: bool,
    pub has_manufacturing_time: bool,
    seller_id: u32,
}

pub struct SellerNemo {
    pub id: u32,
    pub reputation: String,
    pub search_url: String,
}

pub async fn find_nemo(site_param: Option<&String>, mp: Option<&String>, me: Option<&String>, it: Option<&String>, variations: Option<&String>, ic: Option<&String>, cq: Option<&String>) -> Nemo {
    let site = site_param.map_or(String::from("NONE"), |s| s.to_string());
    let item_type = it.map_or(String::from("NONE"), |s| s.to_string());
    let mercado_pago = mp.map_or(String::from("NONE"), |s| s.to_string());
    let mercado_envios = me.map_or(String::from("NONE"), |s| s.to_string());
    let variations = variations.map_or(String::from("NONE"), |s| s.to_string());
    let item_condition = ic.map_or(String::from("NONE"), |s| s.to_string());
    let custom_query = cq.map_or(String::from("NONE"), |s| if s.len() <= 2 { String::from("NONE") } else { s.to_string() });

    let params = get_params(site.as_str(), item_type.as_str(), mercado_pago.as_str(), mercado_envios.as_str(), variations.as_str(), item_condition.as_str(), custom_query.as_str());

    let results = get_items_ids(site.as_str(), params.as_str()).await;

    let search_url = format!("{}?{}", get_host_by_site(site.as_str()), params);

    return map_response_to_nemo(search_url.as_str(), results, site.as_str(), mercado_envios.as_str(), item_type.as_str()).await;
}

async fn map_response_to_nemo(search_url: &str, results: Vec<ResultsResponse>, site: &str, mercado_envios: &str, item_type: &str) -> Nemo {
    let items_ids: Vec<&str> = results.iter().map(|result| { result.id.as_str() }).collect();
    let mut sellers_types = HashMap::new();

    let items: Vec<ItemNemo> = future::join_all(items_ids.into_iter().map(|item_id| async move {
        let item = get_item(item_id).await;

        ItemNemo {
            id: item.id,
            permalink: item.permalink,
            has_puis: item.has_puis,
            has_manufacturing_time: item.has_manufacturing_time,
            seller_id: item.seller_id,
        }
    })).await.into_iter().filter(|item| filter_item_nemo(item, mercado_envios, item_type)).collect();

    let sellers_ids: HashSet<u32> = items.iter().map(|item| { item.seller_id }).collect();

    let sellers = future::join_all(sellers_ids.into_iter().map(|seller_id| async move {
        let seller = get_seller(seller_id).await;

        SellerNemo {
            id: seller.id,
            reputation: seller.reputation,
            search_url: get_seller_search(site, seller.id),
        }
    })).await;

    for seller in sellers {
        let sellers_types_key = seller.reputation.as_str().to_string();
        sellers_types.entry(sellers_types_key).or_insert_with(Vec::new).push(seller)
    }

    Nemo {
        search_url: search_url.to_string(),
        items,
        sellers_types
    }
}