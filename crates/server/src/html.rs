use actix_web::{*};
use crate::const_params::{*};

#[derive(Debug)]
pub struct HtmlRensponceConfig {
  pub port: i32,
  pub host: String
}

pub fn respond_html() -> impl Responder {
  let description = "";
  let allow_publish = false;

  let robots = if allow_publish { "index,follow" } else { "noindex,nofollow" };
  HttpResponse::Ok()
    .content_type("text/html")
    .body(format!(
r###"
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width,initial-scale=1.0">
    <meta name="description" content="{description}">
    <meta name="format-detection" content="email=no,telephone=no,address=no">
    <meta name="robots" content="{robots}">
    <link rel="icon" href="{root}/favicon.ico">
    <title></title>
  </head>
  <body>
    <noscript>
      <strong>We're sorry but doesn't work properly without JavaScript enabled. Please enable it to continue.</strong>
    </noscript>
    <script src="{root}/js/strattera.js"></script>
  </body>
</html>
"###,
  root=format!("./{}/", RESOURCE_ROOT_DIR_NAME),
  description=description,
  robots=robots,
))
  // TODO: OGP
}
