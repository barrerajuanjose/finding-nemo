mod nemo;

use actix_web::{ web, App, HttpRequest, HttpResponse, HttpServer, Result };
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use std::env;

async fn index(req: HttpRequest) -> Result<HttpResponse> {
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
                             params.get("it"));

    Ok(HttpResponse::Ok().json(NemoResponse {
        search_url: nemo.search_url
    }))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| {
        App::new()
            .route("/search", web::get().to(index))
        })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}

#[derive(Serialize, Deserialize)]
struct NemoResponse {
    search_url: String,
}