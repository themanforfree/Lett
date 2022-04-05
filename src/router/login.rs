use super::{HyperReq, HyperRes};
use hyper::Method;

fn check_login(username: &str, password: &str) -> bool {
    username == "admin" && password == "admin"
}

pub async fn handle(req: HyperReq) -> HyperRes {
    match req.method() {
        &Method::GET => {
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
            HyperRes::new(hyper::Body::from(from_html))
        }
        &Method::POST => {
            let body = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let query =
                serde_urlencoded::from_bytes::<Vec<(String, String)>>(body.as_ref()).unwrap();
            let username = &query.iter().find(|(k, _)| k == "username").unwrap().1;
            let password = &query.iter().find(|(k, _)| k == "password").unwrap().1;
            if check_login(username, password) {
                let mut res = HyperRes::new(hyper::Body::empty());
                *res.status_mut() = hyper::StatusCode::FOUND;
                res.headers_mut().insert(
                    hyper::header::LOCATION,
                    hyper::header::HeaderValue::from_str("/admin").unwrap(),
                );
                res
            } else {
                HyperRes::new(hyper::Body::from("login failed"))
            }
        }
        _ => HyperRes::new(hyper::Body::from("Unsupported method")),
    }
}
