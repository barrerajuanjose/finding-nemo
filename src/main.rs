mod nemo;

use actix_web::{ web, App, HttpRequest, HttpResponse, HttpServer, Result };
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

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
    HttpServer::new(|| {
        App::new()
            .route("/search", web::get().to(index))
        })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}

#[derive(Serialize, Deserialize)]
struct NemoResponse {
    search_url: String,
}