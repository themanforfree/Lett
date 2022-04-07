use crate::database::models::establish_connection;
use crate::database::models::session::{self, Session};
use hyper::{header, Body, Method, Request, Response, StatusCode};
use serde::Deserialize;

#[derive(Deserialize)]
struct Params<'a> {
    username: &'a str,
    password: &'a str,
}

fn check_login(username: &str, password: &str) -> bool {
    username == "admin" && password == "admin"
}

pub(crate) async fn handle(req: Request<Body>) -> Option<Response<Body>> {
    match *req.method() {
        Method::GET => {
            let from_html = r#"
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta http-equiv="X-UA-Compatible" content="IE=edge" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Document</title>
            </head>
            <body>
                <form action="/login" method="post">
                    <h2>Login</h2>
                    <input type="text" name="username" />
                    <br />
                    <input type="password" name="password" />
                    <br />
                    <button type="submit">submit</button>
                </form>
            </body>
        </html>
        "#;
            Some(Response::new(hyper::Body::from(from_html)))
        }
        Method::POST => {
            let body = hyper::body::to_bytes(req.into_body()).await.ok()?;
            let query = serde_urlencoded::from_bytes::<Params>(&body).ok()?;
            if check_login(query.username, query.password) {
                let session = Session::new();

                let mut res = Response::new(Body::empty());
                *res.status_mut() = StatusCode::FOUND;
                res.headers_mut().insert(
                    header::LOCATION,
                    header::HeaderValue::from_str("/admin").unwrap(),
                );
                match session.to_cookie() {
                    Ok(cookie) => {
                        session::insert(&establish_connection(), &session).unwrap_or_else(|e| {
                            panic!("{}", e);
                        });
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
