mod mrray;
mod marlin;
mod search;
mod item;
mod seller;

use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{ web, App, HttpRequest, Error, HttpResponse, HttpServer, Result };
use serde::{ Serialize };
use std::collections::HashMap;
use std::env;
use crate::marlin::Nemo;
use std::fs::File;
use std::io::Read;
use actix_files;

use handlebars::Handlebars;

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
                                 params.get("variations"),
                                 params.get("ic"),
                                 params.get("cq")).await;

    Ok(HttpResponse::Ok().json(map_nemo_to_dto(nemo)))
}

async fn index(_req: HttpRequest) -> Result<HttpResponse> {
    let current_dir = env::current_dir().unwrap();
    let index_path = format!("{}{}", current_dir.display(), "/html/index.html");
    let mut file = File::open(index_path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(contents))
}

async fn files(req: HttpRequest) -> Result<actix_files::NamedFile, Error> {
    let current_dir = env::current_dir().unwrap();
    let path_str = format!("{}/static/{}", current_dir.display(), req.match_info().query("filename"));
    let file = actix_files::NamedFile::open(path_str)?;

    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}

async fn used_cars(hb: web::Data<Handlebars<'_>>, _req: HttpRequest) -> Result<HttpResponse> {
    let data = "pepe";

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(hb.render("used_cars", &data).unwrap()))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Handlebars uses a repository for the compiled templates. This object must be
    // shared between the application threads, and is therefore passed to the
    // Application Builder as an atomic reference-counted pointer.
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("used_cars", "./html/templates/used_cars.html")
        .unwrap();

    let handlebars_ref = web::Data::new(handlebars);

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .route("/", web::get().to(index))
            .route("/search", web::get().to(search))
            .route("/static/{filename:.*}", web::get().to(files))
            .route("/autos-usados-mercadolibre-ultima-oportunidad", web::get().to(used_cars))
        })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}

#[derive(Serialize)]
struct NemoDto {
    search_url: String,
    items: Vec<ItemDto>,
    sellers_types: Vec<SellerTypeDto>,
}

#[derive(Serialize)]
struct ItemDto {
    id: String,
    permalink: String,
}

#[derive(Serialize)]
struct SellerTypeDto {
    id: String,
    sellers: Vec<SellerDto>,
}

#[derive(Serialize)]
struct SellerDto {
    id: u32,
    reputation: String,
    search_url: String,
}

fn map_nemo_to_dto(nemo: Nemo) -> NemoDto {
    let mut sellers_types = Vec::new();
    let items:Vec<ItemDto> = nemo.items.iter()
        .take(20)
        .map(|result|
            ItemDto {
                id: result.id.to_string(),
                permalink: result.permalink.to_string(),
            })
        .collect();

    for seller_type_entry in nemo.sellers_types {
        let sellers = seller_type_entry.1
            .iter()
            .take(5)
            .map(|seller_nemo| {
                SellerDto {
                    id: seller_nemo.id,
                    reputation: seller_nemo.reputation.to_string(),
                    search_url: seller_nemo.search_url.to_string(),
                }
            })
            .collect();

        sellers_types.push(SellerTypeDto {
            id: seller_type_entry.0,
            sellers,
        });
    }

    return NemoDto {
        search_url: nemo.search_url,
        items,
        sellers_types
    }
}