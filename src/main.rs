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

#[derive(Serialize, Deserialize)]
struct Directory {
    dir_name: String,
    dir_entries: Vec<String>
}

#[get("/")]
fn index() -> &'static str {
    "Return index.html"
}

#[get("/showdir")]
fn get_dir() -> Json<Directory> {
   
    let dir_name = String::from(util::curr_dir().to_str().unwrap());
    let mut entries: Vec<String> = Vec::new();

    let dir = util::fs::read::read_directory(&util::curr_dir());
    for entry in dir {
        match entry {
            Ok (e) => {
                let path = &e.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                entries.push(String::from(file_name));
            },
            Err(why) => {
                println!("! {:?}", why.kind());
            },
        }
    }
    return Json(Directory { dir_name: dir_name, dir_entries: entries});
}

fn main() {
    rocket::ignite().mount("/", routes![index, get_dir]).launch();
}