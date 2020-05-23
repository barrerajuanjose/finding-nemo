#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod nemo;

use rocket_contrib::json::Json;
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[get("/search?<site>&<mp>&<me>&<it>")]
fn index(site: Option<String>, mp: Option<String>, me: Option<String>, it: Option<String>) -> Json<nemo::Nemo> {
    Json(nemo::find_it(site, mp, me, it))
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

// This is what #[derive(Serialize)] would generate.
impl Serialize for nemo::Nemo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut s = serializer.serialize_struct("Nemo", 1)?;
        s.serialize_field("search_url", &self.search_url)?;
        s.end()
    }
}