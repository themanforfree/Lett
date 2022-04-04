use super::{HyperReq, HyperRes};

pub async fn handle(_req: HyperReq) -> HyperRes {
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
        <form action="/new" method="post">
            <input type="text" name="title" />
            <input type="text" name="content" />
            <button type="submit">submit</button>
        </form>
    </body>
</html>
"#;
    HyperRes::new(hyper::Body::from(from_html))
}
