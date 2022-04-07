use crate::database::models::{establish_connection, session};
use hyper::{header, Body, Request, Response, StatusCode};

pub(crate) async fn handle(req: Request<Body>) -> Option<Response<Body>> {
    match session::get_from_request(&establish_connection(), &req) {
        Some(s) if s.check_expiration() => {
            log::debug!("Request admin page success: {:?}", s);
            let admin_page = r#"
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="UTF-8" />
                    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>Document</title>
                </head>
                <body>
                    <form action="/new" method="post">
                        <h2>New Article</h2>
                        <input type="text" name="title" />
                        <input type="text" name="content" />
                        <button type="submit">submit</button>
                    </form>
                    <form action="/delete" method="post">
                        <h2>Delete Article</h2>
                        <input type="text" name="aid" />
                        <button type="submit">submit</button>
                    </form>
                </body>
            </html>
            "#;
            Some(Response::new(Body::from(admin_page)))
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
