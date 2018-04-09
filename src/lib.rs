//! WEB MMO Main routine.
#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate tera;

extern crate dotenv;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub mod db;
pub mod models;
pub mod schema;
