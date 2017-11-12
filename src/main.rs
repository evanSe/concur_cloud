#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

struct User {
	username: string,
	password: string,
}

#[get("/")]
fn index() -> &'static str {
    "Return index.html"
}

#[post("/login", format = "application/json", data = "<login>")]
fn login(login: Json<User>) -> T {
    "HeyYouJustTriedToLogin"
}

fn main() {
    rocket::ignite().mount("/", routes![index, login]).launch();
}