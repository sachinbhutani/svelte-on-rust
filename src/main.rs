#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::JsonValue;

// for base route api
#[get("/")]
fn api() -> String {
    format!("Hello, from Rust  \n Hit the back button to conitnue")
}

#[get("/message", rank = 1)]
fn json_message() ->  JsonValue{
    json!({
        "app" : "svelte-on-rust",
        "version" : "0.0.1",
        "status": "Feeling Good"
    })
}

#[get("/common", rank = 1)]
fn common() ->  JsonValue{
    json!({
        "app" : "svelte-on-rust",
        "version" : "0.0.1",
        "status": "Feeling Good"
    })
}

fn mount_rocket() -> rocket::Rocket{
    rocket::ignite()
    .mount("/api",routes![api,json_message])
    .mount("/app",routes![common])
    .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")))

}

fn main() {
    mount_rocket()
    .launch();
}