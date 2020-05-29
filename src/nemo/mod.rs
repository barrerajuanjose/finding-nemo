use crate::search::get_host_by_site;
use crate::search::get_items_ids;

pub struct Nemo {
    pub search_url: String,
    pub item_ids: Vec<String>,
}

pub async fn find_it(site_param: Option<&String>, mp: Option<&String>, me: Option<&String>, it: Option<&String>) -> Nemo {
    let site = site_param.map_or(String::from("NONE"), |s| s.to_string());
    let item_type = it.map_or(String::from("NONE"), |s| s.to_string());
    let mercado_pago = mp.map_or(String::from("NONE"), |s| s.to_string());
    let mercado_envios = me.map_or(String::from("NONE"), |s| s.to_string());

    let mut params = format!("?q={}", resolve_q(site.as_str(), item_type.as_str(), mercado_pago.as_str()));

    match mercado_pago.as_str() {
        "psj" => params += "&installments=no_interest",
        "installments" => params += "&installments=yes",
        "highestprice" => params += "&sort=price_desc",
        "lowestprice" => params += "&sort=price_asc",
        _ => println!("{}", "Not value".to_string()),
    }

    match item_type.as_str() {
        "to" => params += "&official_store=all",
        "bs" => params += "&power_seller=yes",
        "video" => params += "&has_video=true",
        _ => println!("{}", "Not value".to_string()),
    }

    match mercado_envios.as_str() {
        "me1" | "me2" => params += "&shipping=mercadoenvios",
        "full" => params += "&shipping=fulfillment",
        _ => println!("{}", "Not value".to_string()),
    }

    let item_ids = get_items_ids(site.as_str(), params.as_str()).await;

    let search_url = format!("{}{}", get_host_by_site(site), params);

    return Nemo {
        search_url,
        item_ids,
    }
}

fn resolve_q(site: &str, it: &str, mp: &str) -> String {
    let (q_highest_price, q_lowest_price) = if site == "MLB" {
        ("ferramientas", "livros")
    } else {
        ("herramientas", "libros")
    };

    if it == "granel" {
        String::from("piso-vinilico")
    } else if mp == "highestprice" {
        q_highest_price.to_string()
    } else if mp == "lowestprice" {
        q_lowest_price.to_string()
    } else {
        String::from("celulares")
    }
}