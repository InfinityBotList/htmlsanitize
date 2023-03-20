use axum::{routing::post, Router, http::{self}};
use pulldown_cmark::{Options, Parser, html::push_html};
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", post(handler)).layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers([http::header::CONTENT_TYPE, http::header::ACCEPT, http::header::HeaderName::from_static("x-client")]),
    );

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 5810));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(
    body: String,
) -> String {
    // Parse to HTML
    let options = Options::all();
    let md_parse = Parser::new_ext(&body, options);
    let mut html = String::new();
    push_html(&mut html, md_parse);

    ammonia::Builder::new()
        .rm_clean_content_tags(&["style", "iframe"])
        .add_tags(&[
            "span", "img", "video", "iframe", "style", "p", "br", "center", "div", "h1", "h2",
            "h3", "h4", "h5", "section", "article", "lang", "code", "pre", "strong", "em",
        ])
        .add_generic_attributes(&[
            "id",
            "class",
            "style",
            "data-src",
            "data-background-image",
            "data-background-image-set",
            "data-background-delimiter",
            "data-icon",
            "data-inline",
            "data-height",
            "code",
        ])
        .add_tag_attributes("iframe", &["src", "height", "width"])
        .add_tag_attributes(
            "img",
            &[
                "src",
                "alt",
                "width",
                "height",
                "crossorigin",
                "referrerpolicy",
                "sizes",
                "srcset",
            ],
        )
        .clean(&html)
        .to_string()    
}

