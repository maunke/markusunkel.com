use leptos::prelude::*;
use leptos_meta::{Html, Link as MetaLink, Meta, Title};
use leptos_router::components::A;

use crate::SITE_URL;
use crate::markdown::{
    BlockQuote, CellAlignment, CodeBlock, ContentNode, DefinitionItem, DefinitionList, Header,
    HeaderLevel, Image, ImageSource, Link, List, ListItem, MarkdownPage, Table, TableCell, Text,
    TextStyle,
};

#[component]
pub fn MarkdownView(page: MarkdownPage, canonical: String, blog: bool) -> impl IntoView {
    let background = page
        .nodes
        .iter()
        .find_map(|node| match node {
            ContentNode::Figure(figure) => Some(&figure.image),
            ContentNode::Image(image) => Some(image),
            _ => None,
        })
        .and_then(|image| image.gradient.as_deref())
        .map(gradient_style)
        .unwrap_or_default();

    let (og_image, og_width, og_height, og_alt) = page
        .nodes
        .iter()
        .find_map(page_image)
        .and_then(|image| {
            let source = image
                .set
                .webp
                .iter()
                .find(|source| source.width == 1280)
                .or_else(|| image.set.webp.iter().max_by_key(|source| source.width))?;
            let height = (f64::from(image.height) / f64::from(image.width.max(1))
                * f64::from(source.width)) as u32;
            Some((
                format!("{SITE_URL}{}", source.path.strip_prefix("/").unwrap()),
                source.width.to_string(),
                height.to_string(),
                image.alt.clone(),
            ))
        })
        .unwrap_or_default();
    let title = page.metadata.title.clone();
    let description = page.metadata.description.clone();

    let header = blog.then(|| {
        let date = page.metadata.date.map(|d| {
            let date = d.format("%B %-d, %Y").to_string();
            let datetime = d.format("%Y-%m-%d").to_string();
            view! { <time datetime=datetime>{date}</time> }
        });
        view! {
            <header>
                <h1>{title.clone()}</h1>
                {date}
            </header>
        }
    });
    let component_tints = page.nodes.iter().any(
        |node| matches!(node, ContentNode::Component(component) if component.page_image().is_some()),
    );
    let page_tint = (!component_tints).then(|| view! { <Html {..} style=background /> });

    let body = match blog {
        true => {
            view! { <article>{header} <Nodes nodes=page.nodes.to_vec() /></article> }.into_any()
        }
        false => view! { <Nodes nodes=page.nodes.to_vec() /> }.into_any(),
    };
    view! {
        <MetaLink rel="canonical" href=canonical.clone() />
        <Title text=title.clone() />
        <Meta name="title" content=title.clone() />
        <Meta name="description" content=description.clone() />
        <Meta property="og:type" content="website" />
        <Meta property="og:title" content=title />
        <Meta property="og:description" content=description />
        <Meta property="og:url" content=canonical />
        <Meta property="og:image" content=og_image />
        <Meta property="og:image:type" content="image/webp" />
        <Meta property="og:image:width" content=og_width />
        <Meta property="og:image:height" content=og_height />
        <Meta property="og:image:alt" content=og_alt />

        {page_tint}
        {body}
    }
}

pub fn gradient_style(gradient: &[[u8; 4]]) -> String {
    const OPACITY: f32 = 0.04;
    gradient
        .iter()
        .take(4)
        .enumerate()
        .map(|(i, [r, g, b, _a])| format!("--page-gradient-{}:rgba({r},{g},{b},{OPACITY});", i + 1))
        .collect()
}

fn page_image(node: &ContentNode) -> Option<&Image> {
    match node {
        ContentNode::Figure(figure) => Some(&figure.image),
        ContentNode::Image(image) => Some(image),
        ContentNode::Component(component) => component.page_image(),
        _ => None,
    }
}

#[component]
fn Nodes(nodes: Vec<ContentNode>) -> AnyView {
    nodes
        .into_iter()
        .map(|node| view! { <Node node /> })
        .collect_view()
        .into_any()
}

#[component]
fn Node(node: ContentNode) -> AnyView {
    match node {
        ContentNode::Header(Header { level, children }) => {
            let children = view! { <Nodes nodes=children /> };
            match level {
                HeaderLevel::H1 => view! { <h1>{children}</h1> }.into_any(),
                HeaderLevel::H2 => view! { <h2>{children}</h2> }.into_any(),
                HeaderLevel::H3 => view! { <h3>{children}</h3> }.into_any(),
                HeaderLevel::H4 => view! { <h4>{children}</h4> }.into_any(),
                HeaderLevel::H5 => view! { <h5>{children}</h5> }.into_any(),
            }
        }
        ContentNode::Paragraph(paragraph) => view! {
            <p>
                <Nodes nodes=paragraph.children />
            </p>
        }
        .into_any(),
        ContentNode::BlockQuote(BlockQuote { children }) => view! {
            <blockquote>
                <Nodes nodes=children />
            </blockquote>
        }
        .into_any(),
        ContentNode::Link(Link {
            url,
            title,
            children,
            ..
        }) if links_to_file(&url) => view! {
            <a href=url rel="external" title=link_title(title)>
                <Nodes nodes=children />
            </a>
        }
        .into_any(),
        ContentNode::Link(Link {
            url,
            title,
            children,
            ..
        }) if url == "https://mastodon.social/@maunke" => view! {
            <a href=url rel="me" title=link_title(title)>
                <Nodes nodes=children />
            </a>
        }
        .into_any(),
        ContentNode::Link(Link {
            url,
            title,
            children,
            ..
        }) => view! {
            <A href={url} {..} title=link_title(title)>
                <Nodes nodes=children />
            </A>
        }
        .into_any(),
        ContentNode::Text(Text::Raw(text)) => text.into_any(),
        ContentNode::Text(Text::Styled { style, children }) => {
            let children = view! { <Nodes nodes=children /> };
            match style {
                TextStyle::Strong => view! { <strong>{children}</strong> }.into_any(),
                TextStyle::Emphasis => view! { <em>{children}</em> }.into_any(),
                TextStyle::Strikethrough => view! { <s>{children}</s> }.into_any(),
                TextStyle::Subscript => view! { <sub>{children}</sub> }.into_any(),
                TextStyle::Superscript => view! { <sup>{children}</sup> }.into_any(),
                TextStyle::Small => view! { <small>{children}</small> }.into_any(),
            }
        }
        ContentNode::SoftBreak => " ".into_any(),
        ContentNode::HardBreak => view! { <br /> }.into_any(),
        ContentNode::Image(image) => view! { <Picture image block=false /> }.into_any(),
        ContentNode::Figure(figure) => {
            let mut image = figure.image;
            let caption = std::mem::take(&mut image.caption);
            if caption.is_empty() {
                return view! { <Picture image block=true /> }.into_any();
            }
            view! {
                <figure>
                    <Picture image block=true />
                    <figcaption>
                        <span class="fignumber">"Figure "{figure.number + 1}":"</span>
                        <Nodes nodes=caption />
                    </figcaption>
                </figure>
            }
            .into_any()
        }
        ContentNode::List(List { start, items }) => {
            let items = items
                .into_iter()
                .map(|ListItem { task, children }| {
                    let checkbox = task.map(|checked| {
                        view! { <input r#type="checkbox" checked=checked disabled=true /> }
                    });
                    let style = task.map(|_| "list-style:none");
                    view! { <li style=style>{checkbox}<Nodes nodes=children /></li> }
                })
                .collect_view();
            match start {
                Some(start) => {
                    view! { <ol start=(start != 1).then_some(start)>{items}</ol> }.into_any()
                }
                None => view! { <ul>{items}</ul> }.into_any(),
            }
        }
        ContentNode::DefinitionList(DefinitionList { items }) => {
            let items = items
                .into_iter()
                .map(|item| match item {
                    DefinitionItem::Title(children) => view! {
                        <dt>
                            <Nodes nodes=children />
                        </dt>
                    }
                    .into_any(),
                    DefinitionItem::Definition(children) => view! {
                        <dd>
                            <Nodes nodes=children />
                        </dd>
                    }
                    .into_any(),
                })
                .collect_view();
            view! { <dl>{items}</dl> }.into_any()
        }
        ContentNode::Table(Table {
            alignments,
            head,
            rows,
            number,
            caption,
        }) => {
            let head = (!head.iter().all(|cell| {
                cell.children
                .iter()
                .all(|node| matches!(node, ContentNode::Text(Text::Raw(text)) if text.trim().is_empty()))
            }))
            .then(|| {
                let cells = head
                    .into_iter()
                    .enumerate()
                    .map(|(column, TableCell { children })| {
                        view! {
                            <th style=text_align_style(alignments.get(column))>
                                <Nodes nodes=children />
                            </th>
                        }
                    })
                    .collect_view();
                view! {
                    <thead>
                        <tr>{cells}</tr>
                    </thead>
                }
            });
            let rows = rows
                .into_iter()
                .map(|row| {
                    let cells = row
                        .into_iter()
                        .enumerate()
                        .map(|(column, TableCell { children })| {
                            view! {
                                <td style=text_align_style(alignments.get(column))>
                                    <Nodes nodes=children />
                                </td>
                            }
                        })
                        .collect_view();
                    view! { <tr>{cells}</tr> }
                })
                .collect_view();

            if caption.is_empty() {
                return view! {
                    <div style="overflow-x:auto" tabindex="0">
                        <table>{head} <tbody>{rows}</tbody></table>
                    </div>
                }
                .into_any();
            }

            let caption_id = format!("table-{}", number + 1);
            view! {
                <figure>
                    <div
                        style="overflow-x:auto"
                        tabindex="0"
                        role="region"
                        aria-labelledby=caption_id.clone()
                    >
                        <table aria-labelledby=caption_id
                            .clone()>{head} <tbody>{rows}</tbody></table>
                    </div>
                    <figcaption id=caption_id>
                        <b>"Table "{number + 1} ": "</b>
                        <Nodes nodes=caption />
                    </figcaption>
                </figure>
            }
            .into_any()
        }
        ContentNode::CodeBlock(CodeBlock { language, code }) => view! {
            <pre>
                <code class=language.map(|language| format!("language-{language}"))>{code}</code>
            </pre>
        }
        .into_any(),
        ContentNode::Component(component) => component.view(),
        ContentNode::Metadata(_) => ().into_any(),
    }
}

fn link_title(title: String) -> Option<String> {
    (!title.is_empty()).then_some(title)
}

fn links_to_file(url: &str) -> bool {
    let path = url.split(['?', '#']).next().unwrap_or_default();
    path.rsplit('/')
        .next()
        .is_some_and(|segment| segment.contains('.'))
}

fn text_align_style(alignment: Option<&CellAlignment>) -> Option<&'static str> {
    match alignment? {
        CellAlignment::None => None,
        CellAlignment::Left => Some("text-align:left"),
        CellAlignment::Center => Some("text-align:center"),
        CellAlignment::Right => Some("text-align:right"),
    }
}

fn srcset(sources: &[ImageSource]) -> String {
    sources
        .iter()
        .map(|source| format!("{} {}w", source.path, source.width))
        .collect::<Vec<_>>()
        .join(", ")
}

const IMG_SIZES: &str = "(max-width: 640px) calc(100vw - 32px), 608px";

#[component]
pub fn Picture(image: Image, block: bool) -> impl IntoView {
    let Image {
        path,
        alt,
        title,
        lqip,
        set,
        width,
        height,
        number,
        ..
    } = image;
    let title = (!title.is_empty()).then_some(title);
    let picture_style = (!block).then_some("display:inline;margin:0");
    let layout = if block {
        "display:block;margin-inline:auto;"
    } else {
        ""
    };
    let placeholder = lqip
        .map(|lqip| {
            format!(
                "background-image:url({lqip});background-size:cover;background-position:center;background-repeat:no-repeat"
            )
        })
        .unwrap_or_default();
    let style = format!("{layout}max-width:100%;height:auto;{placeholder}");

    let (loading, fetchpriority) = match number {
        0 => ("eager", "high"),
        _ => ("lazy", "auto"),
    };
    let src = set
        .webp
        .iter()
        .find(|source| source.width >= 640)
        .or(set.webp.last())
        .map_or(path, |source| source.path.clone());
    let avif = (!set.avif.is_empty())
        .then(|| view! { <source r#type="image/avif" srcset=srcset(&set.avif) sizes=IMG_SIZES /> });
    let (webp, sizes) = if set.webp.is_empty() {
        (None, None)
    } else {
        (Some(srcset(&set.webp)), Some(IMG_SIZES))
    };
    view! {
        <picture style=picture_style>
            {avif}
            <img
                src=src
                srcset=webp
                sizes=sizes
                alt=alt
                title=title
                style=style
                width=width
                height=height
                loading=loading
                fetchpriority=fetchpriority
            />
        </picture>
    }
}
