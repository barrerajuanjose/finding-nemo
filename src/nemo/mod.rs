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

    let (q_highest_price, q_lowest_price) = if site.as_str() == "MLB" {
        ("ferramientas", "livros")
    } else {
        ("herramientas", "libros")
    };

    let mut params = String::from("");

    match mercado_pago.as_str() {
        "psj" => params += "?q=celulares&installments=no_interest",
        "installments" => params += "?q=celulares&installments=yes",
        "highestprice" => params += format!("?q={}&sort=price_desc", q_highest_price).as_str(),
        "lowestprice" => params += format!("?q={}&sort=price_asc", q_lowest_price).as_str(),
        _ => params += "?q=celulares",
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