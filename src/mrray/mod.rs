use crate::marlin::ItemNemo;

pub fn get_params(site: &str, item_type: &str, mercado_pago: &str, mercado_envios: &str, variations: &str, item_condition: &str, custom_query: &str) -> String {
    let mut params = String::from("");

    if mercado_envios == "cbt" {
        if site == "MLB" {
            params = String::from("official_store=2500")
        } else if site == "MLM" {
            params = String::from("seller_id=329558822")
        }
    } else if item_type == "cpg" {
        match site {
            "MLB" => params = String::from("deal=MLB1960"),
            "MLA" => params = String::from("deal=MLA3935"),
            _ => params = String::from("deal=MLM1943"),
        }
    } else {
        params = format!("q={}", resolve_q(site, item_type, mercado_pago, variations, custom_query));
    }

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
        "video" => params += "&has_video=yes",
        "refurbished" => params += "&ITEM_CONDITION=2230582",
        _ => println!("{}", "Not value".to_string()),
    }

    match mercado_envios {
        "me1" | "me2" => params += "&shipping=mercadoenvios",
        "full" => params += "&shipping=fulfillment",
        _ => println!("{}", "Not value".to_string()),
    }

    match  item_condition {
        "new" => params += "&ITEM_CONDITION=2230284",
        "used" => params += "&ITEM_CONDITION=2230581",
        _ => println!("{}", "Not value".to_string()),
    }

    params
}

pub fn filter_item_nemo(item_nemo: &ItemNemo, mercado_envios: &str, item_type: &str) -> bool {
    if mercado_envios == "puis" && item_type == "manufacturing" {
        item_nemo.has_puis && item_nemo.has_manufacturing_time
    } else if mercado_envios == "puis" {
        item_nemo.has_puis
    } else if item_type == "manufacturing" {
        item_nemo.has_manufacturing_time
    } else {
        true
    }
}

fn resolve_q(site: &str, it: &str, mp: &str, variations: &str, custom_query: &str) -> String {
    let (q_highest_price, q_lowest_price, q_variations_one, q_variations_two, q_variations_more) = if site == "MLB" {
        ("ferramientas", "livros", "oculos", "tenis", "camisetas-masculino")
    } else {
        ("herramientas", "libros", "lentes", "zapatillas", "remeras")
    };

    if custom_query != "NONE" {
        String::from(custom_query)
    } else if it == "manufacturing" {
        String::from("sofa")
    } else if it == "granel" {
        String::from("piso-vinilico")
    } else if mp == "highestprice" {
        q_highest_price.to_string()
    } else if mp == "lowestprice" {
        q_lowest_price.to_string()
    } else if variations == "one" {
        q_variations_one.to_string()
    } else if variations == "two" {
        q_variations_two.to_string()
    } else if variations == "more" {
        q_variations_more.to_string()
    } else {
        String::from("celulares")
    }
}