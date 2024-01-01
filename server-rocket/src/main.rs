#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use rocket::request::Form;
use rocket::Data;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[derive(FromForm)]
struct Task {
    complete: bool,
    description: String,
}

#[post("/file", data = "<task>")]
fn new(task: Option<Form<Task>>) -> &'static str {
    "Hello, world!11"
}

fn main() {
    rocket::ignite().mount("/", routes![hello, file]).launch();
//    rocket::ignite().mount("/", routes![file]).launch();
}
