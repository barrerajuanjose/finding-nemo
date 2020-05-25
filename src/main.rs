mod search;
mod nemo;

use actix_web::{ web, App, HttpRequest, HttpResponse, HttpServer, Result };
use serde::{ Serialize };
use std::collections::HashMap;
use std::env;

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

    let nemo = nemo::find_it(params.get("site"),
                             params.get("mp"),
                             params.get("me"),
                             params.get("it")).await;

    Ok(HttpResponse::Ok().json(NemoResponse {
        search_url: nemo.search_url,
        item_ids: nemo.item_ids,
    }))
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
struct NemoResponse {
    search_url: String,
    item_ids: Vec<String>,
}