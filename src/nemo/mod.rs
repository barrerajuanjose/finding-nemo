use crate::search::get_host_by_site;
use crate::search::get_items_ids;

pub struct Nemo {
    pub search_url: String,
    pub item_ids: Vec<String>,
}

pub async fn find_it(site: Option<&String>, mp: Option<&String>, me: Option<&String>, it: Option<&String>) -> Nemo {
    let mut search_url = get_host_by_site(site.map_or(String::from("NONE"), |s| s.to_string()));

    let item_type = it.map_or(String::from("NONE"), |s| s.to_string());
    let mercado_pago = mp.map_or(String::from("NONE"), |s| s.to_string());
    let mercado_envios = me.map_or(String::from("NONE"), |s| s.to_string());

    match mercado_pago.as_str() {
        "psj" => search_url += "?q=celulares&installments=no_interest",
        "nopsj" => search_url += "?q=celulares&installments=yes",
        "higestprice" => search_url += "?q=herramientas&sort=price_desc",
        "lowerprice" => search_url += "?q=libros&sort=price_asc",
        _ => search_url += "?q?celulares",
    }

    match item_type.as_str() {
        "to" => search_url += "&official_store=all",
        "bs" => search_url += "&power_seller=yes",
        "video" => search_url += "&has_video=true",
        _ => println!("{}", "Not value".to_string()),
    }

    match mercado_envios.as_str() {
        "me1" | "me2" => search_url += "&shipping=mercadoenvios",
        "full" => search_url += "&shipping=fulfillment",
        _ => println!("{}", "Not value".to_string()),
    }

    let item_ids = get_items_ids(site, search_url.as_str()).await;

    return Nemo {
        search_url,
        item_ids,
    }
}