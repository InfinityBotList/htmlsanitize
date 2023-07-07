use pulldown_cmark::{Options, Parser, html::push_html};
use crate::types::HSLink;

pub fn template(
    body: &str,
    vars: Vec<HSLink>,
) -> String {
    let mut template = body.to_string();

    for var in vars {
        if var.name.starts_with("_") {
            continue;
        }

        template = template.replace(&format!("{{{}}}", var.name), &var.value);
    }

    sanitize(&template)
}

pub fn sanitize(
    body: &str,
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

