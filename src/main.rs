#![feature(proc_macro)]

extern crate iron;
extern crate maud;
#[macro_use]
extern crate router;
extern crate staticfile;
extern crate mount;

mod views;

use std::env;
use iron::prelude::*;
use iron::{status};
use staticfile::Static;
use mount::Mount;

const SCRIPT: &'static str = "script";
const STYLE: &'static str = "style";

fn not_found(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::NotFound)))
}

fn main() {
    // create script path
    let mut script_path = env::current_dir().unwrap();
    script_path.push(SCRIPT);

    // create style path
    let mut style_path = env::current_dir().unwrap();
    style_path.push(STYLE);

    // compose view router and hook up middleware
    let view_router = router!(main: get "/" => views::home);
    let mut view_chain = Chain::new(view_router);
    view_chain.link_before(views::ViewTemplate);
    view_chain.link_after(views::ViewTemplate);
    view_chain.link_after(views::ViewAssembler);

    // compose api router
    let api_router = router!(root: any "/" => not_found );

    // mount routers and static file paths
    let mut mount = Mount::new();
    mount
        .mount("/", view_chain)
        .mount("/api/", api_router)
        .mount(SCRIPT, Static::new(script_path))
        .mount(STYLE, Static::new(style_path));

    // start server
    if let Err(why) = Iron::new(mount).http("localhost:3000") {
        println!("Failed to start server: {}", why);
    }
}
