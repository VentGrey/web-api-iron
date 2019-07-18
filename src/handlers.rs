use std::sync::{Arc, Mutex};
use std::io::Read;
use iron::{status, AfterMiddleware, Handler, IronResult, Request, Response};
use iron::headers::ContentType;
use rustc_serialize::json;

use database::Database;
use uuid::Uuid;
use router::Router;
use models::Post;
use std::error::Error;
