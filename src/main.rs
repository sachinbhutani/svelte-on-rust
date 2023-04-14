use rocket::serde::json::{Value, json};
use rocket::serde::{Serialize,Deserialize, json::Json};
use rocket::{Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header,Status};
use rocket::request::{self, Outcome, Request, FromRequest};
use sha2::{Sha256,Digest};
use chrono::Utc;
use std::convert::Infallible;

#[macro_use] extern crate rocket;

fn get_session_id(data: &String) -> String{
    let time = Utc::now().to_rfc3339();
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.update(time);
    let hash = format!("{:x}",hasher.finalize());
    return hash;
}

#[get("/")]
fn api() -> String {
    format!("Hello, from Rust Backend!")
}

#[get("/message", rank = 1)]
fn json_message() -> Value{
    json!({ "app": "svelte-on-rust", "version": "0.2.0", "status": "ok"})
} 

// User struct for login
#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    password: String
}


#[post("/login", format = "json", data = "<user>")]
fn login_user(user: Json<User>) -> Value{
    // should be replaced with databased/auth service logic to generate tokens
    // issue access token from auth service
    let token = get_session_id(&user.username);
    if user.username == user.password {
        json!({ "result" : "success",
                "messge" : "login successfull",
                "token" : token
        })
    }else {
        json!({ "result" : "error", 
                "message": "Invalid Username/Password"
        })
    }
}

#[get("/logout")]
fn logout_user() -> Value{
    // database logic to process logout should be added 
    json!({ "result" : "success",
            "messge" : "logout successfull"
    })
}

struct Token<'r>(&'r str);

#[derive(Debug)]
enum TokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token<'r> {
    type Error = TokenError;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        //replace with server side validation of the session id /auth token to respond 
        let t = req.headers().get_one("x-token");
        print!("Token recieved:{:?}",t);
        match t {
            None => Outcome::Failure((Status::Unauthorized, TokenError::Missing)),
            Some("_") => Outcome::Failure((Status::Unauthorized, TokenError::Missing)),
            Some(key) => Outcome::Success(Token(key)),
            Some(_) => Outcome::Failure((Status::BadRequest, TokenError::Invalid)),
        }
    }
}

//#[get("/sensitive")]
//fn sensitive(key: ApiKey<'_>) -> &'static str {
//    "Sensitive data."
//}

#[get("/secret")]
fn secret(token: Token<'_>) -> Value {
    //print!("session_id: {}",Token);
    json!({"token": token.0, "message":"This is a secret message from server, just for you!"})
}

//catch all OPTIONS requests to trigger CORS
#[options("/<_..>")]
fn all_options() {
}

#[derive(Default, Clone)]
struct CORS {
}

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/api",routes![api,json_message])
    .mount("/api/auth",routes![login_user,logout_user,all_options])
    .mount("/api/secure",routes![secret,all_options])
    .attach(CORS::default())
}