use crate::{
    database::{establish_connection, models::session},
    router::TEMPLATES,
};
use hyper::{header, Body, Request, Response, StatusCode};
use tera::Context;

pub(crate) async fn handle(req: Request<Body>) -> Option<Response<Body>> {
    match session::get_from_request(&establish_connection(), &req) {
        Some(s) if s.check_expiration() => {
            log::debug!("Request admin page success: {:?}", s);
            let body = TEMPLATES
                .get()
                .unwrap()
                .render("admin.html", &Context::new())
                .unwrap();
            Some(Response::new(Body::from(body)))
        }
        _ => {
            log::debug!("Request admin page failed, Redirect to /login");
            let mut res = Response::new(Body::empty());
            *res.status_mut() = StatusCode::FOUND;
            res.headers_mut().insert(
                header::LOCATION,
                header::HeaderValue::from_str("/login").unwrap(),
            );
            Some(res)
        }
    }
}
