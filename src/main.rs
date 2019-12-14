#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

// use rocket::request::Form;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::{JsonValue,Json};


// for base route api
#[get("/")]
fn api() -> String {
    format!("Hello, from Rust  \n Hit the back button to continue")
}

#[get("/message", rank = 1)]
fn json_message() ->  JsonValue{
    json!({
        "app" : "svelte-on-rust",
        "version" : "0.0.1",
        "status": "Feeling Good"
    })
}

// check if user is logged in 
#[get("/checkuser")]
fn check_user(mut cookies: Cookies) -> JsonValue {
    let user_id: String;
    match cookies.get_private("session_id") {
        Some(cookie) => user_id = cookie.value().to_string(),
        None => user_id = '_'.to_string(),
    }
    json!({"user_id": user_id})
}

// common client and server route
#[get("/common", rank = 1)]
fn common() ->  JsonValue{
    json!({
        "app" : "svelte-on-rust",
        "version" : "0.0.1",
        "status": "Feeling Good"
    })
}

// User struct for login
#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    password: String
}

#[post("/login", format = "json", data = "<user>")]
fn login_user(mut cookies: Cookies<'_>, user: Json<User>) -> JsonValue{
    // should be replaced with databased logic and encrypted passwords
    if user.username == user.password {
        cookies.add_private(Cookie::new("session_id", user.into_inner().username));
        json!({ "result" : "success",
                "messge" : "login successfull"
        })
    }else {
        json!({ "result" : "error", 
                "message": "Invalid Username/Password"
        })
    }
}

#[get("/logout")]
fn logout_user(mut cookies: Cookies<'_>) -> JsonValue{
    cookies.remove_private(Cookie::named("session_id"));
    json!({ "result" : "success",
            "messge" : "logout successfull"
    })
}

//secured area content 
#[get("/secure")]
fn secure_content(mut cookies: Cookies<'_>) -> JsonValue{
    // fectch private date for the user based on session_id 
    match cookies.get_private("session_id"){
        Some(_cookie)=> json!({ "result": "success", "message": "this is your ultra secret private data"}),
        None => json!( {"result" : "error", "message": "no cookies for you"}),
    }
}

fn mount_rocket() -> rocket::Rocket{
    rocket::ignite()
    .mount("/api",routes![api,json_message,check_user,secure_content])
    .mount("/auth",routes![login_user,logout_user])
    .mount("/app",routes![common])
    .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")))

}

fn main() {
    mount_rocket()
    .launch();
}