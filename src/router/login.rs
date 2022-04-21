use crate::database::{
    establish_connection,
    models::session::{self, Session},
};
use crate::router::ADMIN_TEMPLATES;
use hyper::{header, Body, Method, Request, Response, StatusCode};
use matchit::Params;
use serde::Deserialize;
use tera::Context;

#[derive(Deserialize)]
struct LoginParams<'a> {
    username: &'a str,
    password: &'a str,
}

fn check_login(username: &str, password: &str) -> bool {
    username == "admin" && password == "admin"
}

pub async fn handle(req: Request<Body>, _params: Params<'_, '_>) -> Option<Response<Body>> {
    match *req.method() {
        Method::GET => {
            log::debug!("Request login page");
            let body = ADMIN_TEMPLATES
                .get()
                .unwrap()
                .render("login.html", &Context::new())
                .unwrap();
            Some(Response::new(hyper::Body::from(body)))
        }
        Method::POST => {
            let body = hyper::body::to_bytes(req.into_body()).await.ok()?;
            let params = serde_urlencoded::from_bytes::<LoginParams>(&body).ok()?;
            log::debug!("Post to login: username = {}", params.username);
            if check_login(params.username, params.password) {
                let session = Session::new();

                let mut res = Response::new(Body::empty());
                *res.status_mut() = StatusCode::FOUND;
                res.headers_mut().insert(
                    header::LOCATION,
                    header::HeaderValue::from_str("/admin").unwrap(),
                );
                match session.to_cookie() {
                    Ok(cookie) => {
                        if let Err(e) = session::insert(&establish_connection(), &session) {
                            log::error!("Failed to insert session: {}", e);
                        }
                        res.headers_mut().insert(
                            header::SET_COOKIE,
                            header::HeaderValue::from_str(&cookie).unwrap(),
                        );
                    }
                    Err(e) => {
                        println!("{}", e)
                    }
                }
                Some(res)
            } else {
                Some(Response::new(hyper::Body::from("login failed")))
            }
        }
        _ => Some(Response::new(hyper::Body::from("Unsupported method"))),
    }
}
