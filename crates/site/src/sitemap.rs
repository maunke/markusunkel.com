use quick_xml::se::to_string;
use serde::Serialize;

use crate::{content::Content, routes::canonical_url};

pub fn sitemap_xml(content: &Content) -> String {
    let mut urls = vec![];
    content.root.pages.keys().for_each(|root_slug| {
        if ["blog", "404"].contains(&root_slug.as_str()) {
            return;
        }
        urls.push(SitemapUrl {
            loc: canonical_url(root_slug),
        });
    });
    content.blog.pages.keys().for_each(|slug| {
        let blog_slug = format!("blog/{slug}");
        urls.push(SitemapUrl {
            loc: canonical_url(&blog_slug),
        });
    });
    let sitemap = Sitemap {
        xmlns: "http://www.sitemaps.org/schemas/sitemap/0.9",
        url: urls,
    };
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>{}"#,
        to_string(&sitemap).unwrap()
    )
}

#[derive(Debug, Serialize)]
#[serde(rename = "urlset")]
struct Sitemap {
    #[serde(rename = "@xmlns")]
    xmlns: &'static str,
    url: Vec<SitemapUrl>,
}

#[derive(Debug, Serialize)]
struct SitemapUrl {
    loc: String,
}
