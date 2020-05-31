use crate::search::get_host_by_site;
use crate::search::get_items_ids;
use crate::mrray::get_params;

pub struct Nemo {
    pub search_url: String,
    pub item_ids: Vec<String>,
}

pub async fn find_nemo(site_param: Option<&String>, mp: Option<&String>, me: Option<&String>, it: Option<&String>) -> Nemo {
    let site = site_param.map_or(String::from("NONE"), |s| s.to_string());
    let item_type = it.map_or(String::from("NONE"), |s| s.to_string());
    let mercado_pago = mp.map_or(String::from("NONE"), |s| s.to_string());
    let mercado_envios = me.map_or(String::from("NONE"), |s| s.to_string());

    let params = get_params(site.as_str(), item_type.as_str(), mercado_pago.as_str(), mercado_envios.as_str());

    let item_ids = get_items_ids(site.as_str(), params.as_str()).await;

    let search_url = format!("{}{}", get_host_by_site(site.as_str()), params);

    return Nemo {
        search_url,
        item_ids,
    }
}