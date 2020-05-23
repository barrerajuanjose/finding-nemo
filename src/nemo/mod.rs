use std::borrow::Borrow;

pub struct Nemo {
    pub search_url: String,
}

pub fn find_it(site: Option<String>, mp: Option<String>, me: Option<String>, kind_option: Option<String>) -> Nemo {
    let mut search = format!("https://api.mercadolibre.com/sites/{}/searchbackend?q=", site.map_or(String::from("NONE"), |s| s));

    let kind = kind_option.map_or(String::from("NONE"), |s| s);
    let mercado_pago = mp.map_or(String::from("NONE"), |s| s);
    let mercado_envios = me.map_or(String::from("NONE"), |s| s);

    match mercado_pago.as_str() {
        "psj" => search += "celulares&installments=no_interest",
        "nopsj" => search += "celulares&installments=yes",
        "higestprice" => search += "herramientas&sort=price_desc",
        "lowerprice" => search += "libros&sort=price_asc",
        _ => search += "celulares",
    }

    match kind.as_str() {
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