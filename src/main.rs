#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
mod routes;

fn main() {
    dotenv::dotenv().ok();

    rocket::ignite().mount("/", routes![routes::root, routes::login, routes::handle_login]).launch();
}
