#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]


extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod util;
use rocket_contrib::{Json};
use std::string::String;
use std::str;
use std::path::PathBuf;
use std::env;
use std::vec::Vec;
#[derive(Serialize, Deserialize)]
struct User {
	username: String,
    password: String,
    paths: Vec<String>
}

#[get("/")]
fn index() -> &'static str {
    "Return index.html"
}

#[post("/showdir", format = "application/json", data = "<user>")]
fn showdir(user: Json<User>) -> Json<User> {
   
    let mut username = user;
    let dir = util::fs::read::read_directory(&util::curr_dir());
    for entry in dir {
        match entry {
            Ok (e) => {
                let path = &e.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                username.paths.push(String::from(file_name));
            },
            Err(why) => {
                println!("! {:?}", why.kind());
            },
        }
    }
    return username;
}

fn main() {
    rocket::ignite().mount("/", routes![index, showdir]).launch();
}