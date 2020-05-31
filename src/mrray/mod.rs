pub fn get_params(site: &str, item_type: &str, mercado_pago: &str, mercado_envios: &str) -> String {
    let mut params = format!("?q={}", resolve_q(site, item_type, mercado_pago));

    match mercado_pago {
        "psj" => params += "&installments=no_interest",
        "installments" => params += "&installments=yes",
        "highestprice" => params += "&sort=price_desc",
        "lowestprice" => params += "&sort=price_asc",
        _ => println!("{}", "Not value".to_string()),
    }

    match item_type {
        "to" => params += "&official_store=all",
        "bs" => params += "&power_seller=yes",
        "video" => params += "&has_video=true",
        _ => println!("{}", "Not value".to_string()),
    }

    match mercado_envios {
        "me1" | "me2" => params += "&shipping=mercadoenvios",
        "full" => params += "&shipping=fulfillment",
        _ => println!("{}", "Not value".to_string()),
    }

    params
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