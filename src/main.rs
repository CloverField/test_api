#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::sync::Mutex;
use std::collections::HashMap;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

type ID = usize;

type MessageMap = Mutex<HashMap<ID, String>>;

#[derive(Serialize, Deserialize)]
struct Message {
    id: Option<ID>,
    contents: String
}

#[post("/<id>", format = "json", data = "<message>")]
fn new(id: ID, message: Json<Message>, map: State<'_, MessageMap>) -> JsonValue {
    let mut hashmap = map.lock().expect("map lock.");
    if hashmap.contains_key(&id) {
        json!({
            "status": "error",
            "reason": "ID exists. Try put."
        })
    } else {
        hashmap.insert(id, message.0.contents);
        json!({ "status": "ok" })
    }
}

#[put("/<id>", format = "json", data = "<message>")]
fn update(id: ID, message: Json<Message>, map: State<'_, MessageMap>) -> Option<JsonValue> {
    let mut hashmap = map.lock().unwrap();
    if hashmap.contains_key(&id) {
        hashmap.insert(id, message.0.contents);
        Some(json!({ "status": "ok" }))
    } else {
        None
    }
}

#[get("/<id>", format = "json")]
fn get(id: ID, map: State<'_, MessageMap>) -> Option<Json<Message>> {
    let hashmap = map.lock().unwrap();
    hashmap.get(&id).map(|contents| {
        Json(Message {
            id: Some(id),
            contents: contents.clone()
        })
    })
}

#[delete("/<id>")]
fn delete(id: ID, map: State<'_, MessageMap>) -> Option<JsonValue> {
    let mut hashmap = map.lock().unwrap();
    if hashmap.contains_key(&id) {
        hashmap.remove(&id);
        Some(json!({ "status": "ok", "message" : format!("Message {} successfully deleted.", id) }))
    } else {
        None
    }
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[get("/")]
fn helloworld() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>/<age>")]
fn helloperson(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/hello/<name>")]
fn hi(name: String) -> String {
    name
}

fn main() {
    rocket::ignite()
        .mount("/message", routes![new, update, get, delete])
        .mount("/", routes![helloworld, helloperson, hi])
        .register(catchers![not_found])
        .manage(Mutex::new(HashMap::<ID, String>::new()))
        .launch();
}
