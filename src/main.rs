#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]


extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod util;
use rocket_contrib::Json;
use std::string::String;
use std::str;
use std::vec::Vec;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use rocket::http::Status;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
    paths: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Directory {
    dir_name: String,
    dir_entries: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct File {
    file_name: String,
    file_contents: Vec<String>,
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("fe/index.html")
}

#[get("/<file..>", rank = 1)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("fe/").join(file)).ok()
}

#[get("/status")]
fn status() -> String {
    String::from("Alive")
}

#[get("/showdir")]
fn get_dir() -> Json<Directory> {
    // get fcurrent dirrectory name
    let dir_name = String::from(util::curr_dir().to_str().unwrap());
    let mut entries: Vec<String> = Vec::new();

    // read directory and get contents
    let dir = util::fs::read::read_directory(&util::curr_dir());
    for entry in dir {
        match entry {
            Ok(e) => if e.path().is_file() {
                let path = &e.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                entries.push(String::from(file_name));
            },
            Err(why) => {
                println!("! {:?}", why.kind());
            }
        }
    }
    return Json(Directory {
        dir_name: dir_name,
        dir_entries: entries,
    });
}

#[get("/file/<name>")]
fn get_file(name: String) -> Option<Json<File>> {
    let current_dir = util::curr_dir().join(&name);

    match util::fs::read::read_file(
        &current_dir
    ) {
        None => {
            None
        }
        Some(f) => {
            let mut buf_reader = BufReader::new(f);
            let mut entries: Vec<String> = Vec::new();
            for line in buf_reader.lines() {
                match line  {
                    Ok(l) => {
                        entries.push(l);
                    },
                    Err(_) => {
                    }
                }
            }
            Some(Json(File {
                file_name: name,
                file_contents: entries,
            }))
        }
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, get_dir, get_file, files, status])
        .launch();
}
