extern crate mime;
extern crate zhelezo as iron;

use std::env;

use iron::prelude::*;
use iron::headers::ContentType;
use iron::status;

// All these variants do the same thing, with more or less options for customization.

fn variant1(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((ContentType::json().0, status::Ok, "{}")))
}

fn variant2(_: &mut Request) -> IronResult<Response> {
    use iron::mime;
    let content_type = mime::APPLICATION_JSON;
    Ok(Response::with((content_type, status::Ok, "{}")))
}

fn variant3(_: &mut Request) -> IronResult<Response> {
    use iron::mime;
    let content_type = "application/json".parse::<mime::Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, "{}")))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let variant_index = if args.len() > 1 { args[1].parse().unwrap() } else { 1 };
    let handler = match variant_index {
        1 => variant1,
        2 => variant2,
        3 => variant3,
        _ => panic!("No such variant"),
    };
    println!("Using variant{}", variant_index);
    Iron::new(handler).http("localhost:3000");
}
