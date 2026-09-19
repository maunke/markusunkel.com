use chrono::Utc;
use quick_xml::se::to_string;
use serde::Serialize;

use crate::{SITE_URL, content::Content, routes::canonical_url};

pub fn rss_xml(content: &Content) -> String {
    let mut items = vec![];
    content.blog.pages.iter().for_each(|(slug, page)| {
        let blog_slug = format!("blog/{slug}");
        let post_url = canonical_url(&blog_slug);
        let metadata = &page.metadata;
        items.push(RssItem {
            title: metadata.title.clone(),
            link: post_url.clone(),
            guid: RssGuid {
                permalink: true,
                guid: post_url,
            },
            description: metadata.description.clone(),
            pub_date: metadata
                .date
                .expect("blog post must have a date")
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
                .to_rfc2822(),
        })
    });
    let rss = Rss {
        channel: RssChannel {
            item: items,
            ..Default::default()
        },
        ..Default::default()
    };
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>{}"#,
        to_string(&rss).unwrap()
    )
}

#[derive(Debug, Serialize)]
#[serde(rename = "rss")]
struct Rss {
    #[serde(rename = "@version")]
    version: &'static str,
    #[serde(rename = "@xmlns:atom")]
    xmlns_atom: &'static str,

    channel: RssChannel,
}

impl Default for Rss {
    fn default() -> Self {
        Self {
            version: "2.0",
            xmlns_atom: "http://www.w3.org/2005/Atom",
            channel: Default::default(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename = "channel")]
struct RssChannel {
    #[serde(rename = "atom:link")]
    atom_link: AtomLink,
    title: &'static str,
    description: &'static str,
    link: &'static str,
    language: &'static str,
    copyright: &'static str,
    #[serde(rename = "lastBuildDate")]
    last_build_date: String,
    item: Vec<RssItem>,
}

impl Default for RssChannel {
    fn default() -> Self {
        Self {
            atom_link: AtomLink::default(),
            title: "Markus Unkel - Blog",
            description: "This is where I write down and share my thoughts, projects and experiences.",
            link: SITE_URL,
            language: "en-US",
            copyright: "Markus Unkel",
            last_build_date: Utc::now().to_rfc2822(),
            item: vec![],
        }
    }
}

#[derive(Debug, Serialize)]
struct AtomLink {
    #[serde(rename = "@href")]
    href: String,
    #[serde(rename = "@rel")]
    rel: &'static str,
    #[serde(rename = "@type")]
    r#type: &'static str,
}

impl Default for AtomLink {
    fn default() -> Self {
        Self {
            href: format!("{SITE_URL}rss.xml"),
            rel: "self",
            r#type: "application/rss+xml",
        }
    }
}

#[derive(Debug, Serialize)]
struct RssItem {
    title: String,
    link: String,
    guid: RssGuid,
    description: String,
    #[serde(rename = "pubDate")]
    pub_date: String,
}

#[derive(Debug, Serialize)]
struct RssGuid {
    #[serde(rename = "@isPermaLink")]
    permalink: bool,
    #[serde(rename = "$text")]
    guid: String,
}
