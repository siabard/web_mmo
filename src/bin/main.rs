//! WEB MMO Main routine.
#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

extern crate diesel;
extern crate tera;

extern crate dotenv;

extern crate serde;
extern crate serde_json;

extern crate web_mmo;

use rocket_contrib::Template;

use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

use diesel::prelude::*;
use web_mmo::models::*;

use tera::Context;

fn get_test() -> Vec<Test> {
    use web_mmo::schema::test::dsl::*;

    let connection = web_mmo::db::get_connection();
    test.load::<Test>(&connection).unwrap()
}

#[get("/")]
fn index() -> Template {
    let mut context = Context::new();
    let tests = get_test();

    context.add("title", "Web으로 만드는 MMO 시작");
    context.add("tests", &tests);
    Template::render("index", context)
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}

fn rocket() -> rocket::Rocket {
    // Web Site 올리기
    rocket::ignite()
        .mount("/", routes![index, files])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
