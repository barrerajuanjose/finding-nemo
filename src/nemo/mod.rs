pub struct Nemo {
    pub search_url: String,
}

pub fn find_it(site: Option<&String>, mp: Option<&String>, me: Option<&String>, it: Option<&String>) -> Nemo {
    let mut search = format!("https://api.mercadolibre.com/sites/{}/search?q=", site.map_or(String::from("NONE"), |s| s.to_string()));

    let item_type = it.map_or(String::from("NONE"), |s| s.to_string());
    let mercado_pago = mp.map_or(String::from("NONE"), |s| s.to_string());
    let mercado_envios = me.map_or(String::from("NONE"), |s| s.to_string());

    match mercado_pago.as_str() {
        "psj" => search += "celulares&installments=no_interest",
        "nopsj" => search += "celulares&installments=yes",
        "higestprice" => search += "herramientas&sort=price_desc",
        "lowerprice" => search += "libros&sort=price_asc",
        _ => search += "celulares",
    }

    match item_type.as_str() {
        "to" => search += "&official_store=all",
        "bs" => search += "&power_seller=yes",
        "video" => search += "&has_video=true",
        _ => println!("{}", "Not value".to_string()),
    }

    match mercado_envios.as_str() {
        "me1" | "me2" => search += "&shipping=mercadoenvios",
        "full" => search += "&shipping=fulfillment",
        _ => println!("{}", "Not value".to_string()),
    }

    return Nemo {
        search_url: search.to_string()
    };
}