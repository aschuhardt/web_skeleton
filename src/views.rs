use std::net::IpAddr;
use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap, status};
use maud::{html, PreEscaped, DOCTYPE};

/// Stores the rendered HTML header fragment
struct Header;
impl typemap::Key for Header { type Value = String; }

/// Stores the rendered HTML body fragment
struct Body;
impl typemap::Key for Body { type Value = String; }

/// Stores the rendered HTML footer fragment
struct Footer;
impl typemap::Key for Footer { type Value = String; }

/// Provides middleware for rendering and caching response header and footer
/// HTML fragments
pub struct ViewTemplate;

/// Renders the response's header HTML fragment and caches it in the request's
/// key-value store
impl BeforeMiddleware for ViewTemplate {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let markup = html! {
            p {
                "header"
            }
        };
        req.extensions.insert::<Header>(markup.into_string());
        Ok(())
    }
}

/// Renders the response's footer HTML fragment and caches it in the request's
/// key-value store
impl AfterMiddleware for ViewTemplate {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let markup = html! {
            p {
                "footer"
            }
        };
        req.extensions.insert::<Footer>(markup.into_string());
        Ok(res)
    }
}

/// Provides middleware that assembles the response's header, body, and footer HTML
/// fragments into the final response's view document 
pub struct ViewAssembler;

/// Assembles the request's stored HTML fragments into the final view doc
impl AfterMiddleware for ViewAssembler {
    fn after(&self, req: &mut Request, _: Response) -> IronResult<Response> {
        // retrieve HTML fragments from key-value store
        let header = req.extensions.get::<Header>().unwrap();
        let body = req.extensions.get::<Body>().unwrap();
        let footer = req.extensions.get::<Footer>().unwrap();

        // assemble pre-rendered HTML strings into final markup doc
        let markup = html! {
            (DOCTYPE)
            html {
                head {
                    title { "Title" }
                }
                body {
                    div#header {
                        (PreEscaped(header))
                    }
                    br;
                    div#body {
                        (PreEscaped(body))
                    }
                    br;
                    div#footer {
                        (PreEscaped(footer))
                    }
                }
            }
        };
        
        Ok(Response::with((status::Ok, markup)))
    }
}

pub fn home(req: &mut Request) -> IronResult<Response> {
    let ip = match req.remote_addr.ip() {
       IpAddr::V4(ipv4) => format!("{}", ipv4),
       IpAddr::V6(ipv6) => format!("{}", ipv6),
    };

    let markup = html! {
        p#address {
            "Your IP address is: " (ip)
        }
    };
    
    req.extensions.insert::<Body>(markup.into_string());

    Ok(Response::with((status::Ok)))
}
