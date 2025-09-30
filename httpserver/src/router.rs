use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{httprequest, httprequest::HttpRequest};
use std::io::prelude::*;

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        let resp = match req.method {
            httprequest::Method::Get => match req.resource {
                httprequest::Resource::Path(ref s) => {
                    let route: Vec<&str> = s.split('/').collect();
                    match route[1] {
                        "api" => WebServiceHandler::handle(&req),
                        _ => StaticPageHandler::handle(&req),
                    }
                }
            },
            _ => PageNotFoundHandler::handle(&req),
        };
        let _ = resp.send_response(stream);
    }
}
