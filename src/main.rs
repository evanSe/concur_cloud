#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]


extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod routing;
mod util;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                routing::index,
                routing::check_in,
                routing::files,
                routing::get_dir,
                routing::get_file,
            ],
        )
        .launch();
}
