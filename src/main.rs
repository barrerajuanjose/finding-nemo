mod mrray;
mod marlin;
mod search;
mod item;
mod seller;

use actix_web::{ web, App, HttpRequest, HttpResponse, HttpServer, Result };
use serde::{ Serialize };
use std::collections::HashMap;
use std::env;
use crate::marlin::Nemo;

async fn search(req: HttpRequest) -> Result<HttpResponse> {
    let params: HashMap<String, String> = req
        .uri()
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_else(HashMap::new);

    let nemo = marlin::find_nemo(params.get("site"),
                                 params.get("mp"),
                                 params.get("me"),
                                 params.get("it"),
                                    params.get("variations")).await;

    Ok(HttpResponse::Ok().json(map_nemo_to_dto(nemo)))
}

async fn index(_req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/examples.html")))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/search", web::get().to(search))
        })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}

#[derive(Serialize)]
struct NemoDto {
    search_url: String,
    items: Vec<ItemDto>,
}

#[derive(Serialize)]
struct ItemDto {
    id: String,
    permalink: String,
}

fn map_nemo_to_dto(nemo: Nemo) -> NemoDto {
    let mut items = Vec::new();

    for result in nemo.items {
        items.push(ItemDto {
            id: result.id,
            permalink: result.permalink
        });
    }

    return NemoDto {
        search_url: nemo.search_url,
        items,
    }
}