mod models;
mod database;
mod handlers;

use model::*;
use database::Database;
use handlers::*;

use iron::prelude::Chain;
use iron::Iron;
use router::Router;
use logger::Logger;
use uuid::Uuid;

fn main() {
    println!("Hello, world!");
}
