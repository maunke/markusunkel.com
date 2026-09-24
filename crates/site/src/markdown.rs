#[cfg(feature = "ssr")]
use crate::{Error, Result};
use chrono::NaiveDate;
use leptos::prelude::AnyView;
#[cfg(feature = "ssr")]
use pulldown_cmark::{
    Alignment, CodeBlockKind, Event, HeadingLevel, LinkType, MetadataBlockKind, Options, Parser,
    Tag, TagEnd,
};
#[cfg(feature = "ssr")]
use std::path::Path;
use std::{fmt::Display, sync::Arc};

#[cfg(feature = "ssr")]
pub fn content_dir() -> std::path::PathBuf {
    Path::new("content").into()
}

#[derive(Debug, Clone)]
pub enum ContentNode {
    Header(Header),
    Text(Text),
    Link(Link),
    SoftBreak,
    HardBreak,
    Paragraph(Paragraph),
    BlockQuote(BlockQuote),
    Metadata(Metadata),
    Image(Image),
    Figure(Figure),
    List(List),
    DefinitionList(DefinitionList),
    Table(Table),
    CodeBlock(CodeBlock),
    Component(Arc<dyn CustomComponent>),
}

impl ContentNode {
    #[cfg(feature = "ssr")]
    fn try_from_event(event: Event, parser: &mut Parser, tracer: &mut PageTracer) -> Result<Self> {
        let node = match event {
            Event::Start(tag) => match tag {
                Tag::Heading { level, .. } => {
                    Self::Header(Header::try_from_parser(level, parser, tracer)?)
                }
                Tag::Paragraph => {
                    let mut paragraph = Paragraph::try_from_parser(parser, tracer)?;
                    if let [Self::Image(_)] = paragraph.children.as_slice() {
                        let Self::Image(image) = paragraph.children.remove(0) else {
                            return Err(Error::InvalidMarkdown("must be an image"));
                        };
                        Self::Figure(Figure {
                            image,
                            number: tracer.fig(),
                        })
                    } else {
                        Self::Paragraph(paragraph)
                    }
                }
                Tag::Link {
                    link_type,
                    dest_url,
                    title,
                    ..
                } => Self::Link(Link::try_from_parser(
                    link_type,
                    title.into(),
                    dest_url.into_string(),
                    parser,
                    tracer,
                )?),
                Tag::Strong => Self::Text(Text::try_from_parser(
                    Some(TextStyle::Strong),
                    None,
                    parser,
                    tracer,
                )?),
                Tag::Emphasis => Self::Text(Text::try_from_parser(
                    Some(TextStyle::Emphasis),
                    None,
                    parser,
                    tracer,
                )?),
                Tag::Subscript => Self::Text(Text::try_from_parser(
                    Some(TextStyle::Subscript),
                    None,
                    parser,
                    tracer,
                )?),
                Tag::Superscript => Self::Text(Text::try_from_parser(
                    Some(TextStyle::Superscript),
                    None,
                    parser,
                    tracer,
                )?),
                Tag::Strikethrough => Self::Text(Text::try_from_parser(
                    Some(TextStyle::Strikethrough),
                    None,
                    parser,
                    tracer,
                )?),
                Tag::MetadataBlock(block) => {
                    Self::Metadata(Metadata::try_from_parser(block, parser)?)
                }
                Tag::Image {
                    dest_url, title, ..
                } => Self::Image(Image::try_from_parser(
                    dest_url.into_string(),
                    title.into_string(),
                    parser,
                    tracer,
                )?),
                Tag::BlockQuote(_) => {
                    Self::BlockQuote(BlockQuote::try_from_parser(parser, tracer)?)
                }
                Tag::CodeBlock(kind) => Self::try_from_code_block(kind, parser, tracer)?,
                Tag::List(start) => Self::List(List::try_from_parser(start, parser, tracer)?),
                Tag::DefinitionList => {
                    Self::DefinitionList(DefinitionList::try_from_parser(parser, tracer)?)
                }
                Tag::Table(alignments) => {
                    Self::Table(Table::try_from_parser(alignments, parser, tracer)?)
                }
                _ => return Err(Error::InvalidMarkdown("Unsupported start tag")),
            },
            Event::Text(text) => Self::Text(Text::try_from_parser(
                None,
                Some(text.into_string()),
                parser,
                tracer,
            )?),
            Event::SoftBreak => Self::SoftBreak,
            Event::HardBreak => Self::HardBreak,
            Event::InlineHtml(html) if html_tag(&html).as_deref() == Some("small") => Self::Text(
                Text::try_from_parser(Some(TextStyle::Small), None, parser, tracer)?,
            ),
            Event::InlineHtml(_) | Event::Html(_) => {
                return Err(Error::InvalidMarkdown("unsupported html, only <small> is"));
            }
            Event::End(..) => return Err(Error::InvalidMarkdown("unexpected end tag")),
            _ => return Err(Error::InvalidMarkdown("unsupported event")),
        };
        Ok(node)
    }

    #[cfg(feature = "ssr")]
    fn try_from_code_block(
        kind: CodeBlockKind,
        parser: &mut Parser,
        tracer: &mut PageTracer,
    ) -> Result<Self> {
        let info = match kind {
            CodeBlockKind::Fenced(info) => info.into_string(),
            CodeBlockKind::Indented => String::new(),
        };

        let mut body = String::new();
        while let Some(event) = parser.next() {
            match event {
                Event::Text(text) => body.push_str(&text),
                Event::End(TagEnd::CodeBlock) => break,
                _ => return Err(Error::InvalidMarkdown("code block contains text only")),
            }
        }

        let node = match parse_component_info(&info) {
            Some((name, params)) => {
                let Some((_, parse)) = CUSTOM_COMPONENTS.iter().find(|(known, _)| *known == name)
                else {
                    return Err(Error::UnknownComponent(name));
                };
                Self::Component(parse(params, &body, tracer)?)
            }
            None => {
                let language = info.split_whitespace().next().map(str::to_string);
                Self::CodeBlock(CodeBlock {
                    language,
                    code: body,
                })
            }
        };
        Ok(node)
    }
}

#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub language: Option<String>,
    pub code: String,
}

pub trait CustomComponent: std::fmt::Debug + Send + Sync + 'static {
    #[cfg(feature = "ssr")]
    fn try_from_params(params: Params, body: &str, tracer: &mut PageTracer) -> Result<Self>
    where
        Self: Sized;

    fn view(&self) -> AnyView;

    fn page_image(&self) -> Option<&Image> {
        None
    }
}

#[cfg(feature = "ssr")]
const CUSTOM_COMPONENTS: &[(&str, ParseFn)] = &[
    ("blog", parse_component::<crate::components::BlogPosts>),
    (
        "random_image",
        parse_component::<crate::components::RandomImage>,
    ),
];

#[cfg(feature = "ssr")]
type ParseFn = fn(Params, &str, &mut PageTracer) -> Result<Arc<dyn CustomComponent>>;

#[cfg(feature = "ssr")]
fn parse_component<C: CustomComponent>(
    params: Params,
    body: &str,
    tracer: &mut PageTracer,
) -> Result<Arc<dyn CustomComponent>> {
    Ok(Arc::new(C::try_from_params(params, body, tracer)?))
}

#[cfg(feature = "ssr")]
fn parse_component_info(info: &str) -> Option<(String, Params)> {
    let inner = info.trim().strip_prefix('{')?.strip_suffix('}')?.trim();
    let (name, rest) = inner.split_once(char::is_whitespace).unwrap_or((inner, ""));
    (!name.is_empty()).then(|| (name.to_string(), Params::parse(rest)))
}

#[cfg(feature = "ssr")]
pub fn parse_body(body: &str, tracer: &mut PageTracer) -> Result<Vec<ContentNode>> {
    let mut parser = Parser::new_ext(body, Options::all());
    let mut children = Vec::new();
    while let Some(event) = parser.next() {
        children.push(ContentNode::try_from_event(event, &mut parser, tracer)?);
    }
    Ok(children)
}

#[cfg(feature = "ssr")]
#[derive(Debug)]
pub struct Params(Vec<(String, String)>);

#[cfg(feature = "ssr")]
impl Params {
    fn parse(input: &str) -> Self {
        let mut params = Vec::new();
        let mut rest = input.trim_start();
        while let Some((key, tail)) = rest.split_once('=') {
            let tail = tail.trim_start();
            let (value, tail) = match tail.strip_prefix('"') {
                Some(quoted) => quoted.split_once('"').unwrap_or((quoted, "")),
                None => tail.split_once(char::is_whitespace).unwrap_or((tail, "")),
            };
            params.push((key.trim().to_string(), value.to_string()));
            rest = tail.trim_start();
        }
        Self(params)
    }

    pub fn take(&mut self, key: &str) -> Option<String> {
        let index = self.0.iter().position(|(name, _)| name == key)?;
        Some(self.0.remove(index).1)
    }

    pub fn finish(self, component: &str) -> Result<()> {
        match self.0.into_iter().next() {
            Some((param, _)) => Err(Error::UnknownParam {
                component: component.to_string(),
                param,
            }),
            None => Ok(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub title: String,
    pub description: String,
    pub date: Option<NaiveDate>,
}

impl Metadata {
    #[cfg(feature = "ssr")]
    fn try_from_parser(block: MetadataBlockKind, parser: &mut Parser) -> Result<Self> {
        use std::collections::HashMap;

        if block == MetadataBlockKind::PlusesStyle {
            return Err(Error::InvalidMarkdown(
                "pluses metadata style is unsupported",
            ));
        }
        let Some(Event::Text(content)) = parser.next() else {
            return Err(Error::InvalidMarkdown("metadata must have content"));
        };
        if Some(Event::End(TagEnd::MetadataBlock(block))) != parser.next() {
            return Err(Error::InvalidMarkdown(
                "missing metadata tag end after content",
            ));
        }

        let content = content.to_string();
        let mut metadata = HashMap::new();
        content
            .lines()
            .filter(|row| !row.trim().is_empty())
            .map(|row| row.splitn(2, ':').collect::<Vec<_>>())
            .for_each(|kv| {
                let k = kv.get(0).expect("key must be given").trim();
                let v = kv.get(1).expect("value must be given").trim();
                metadata.insert(k, v);
            });

        let title = metadata
            .get("title")
            .expect("metadata: title not provided")
            .to_string();
        let description = metadata
            .get("description")
            .expect("metadata: description not provided")
            .to_string();
        let date: Option<NaiveDate> = metadata.get("date").map(|t| {
            t.parse()
                .expect("metadata: date is not correctly formatted")
        });

        Ok(Metadata {
            title,
            description,
            date,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Figure {
    pub image: Image,
    pub number: usize,
}

#[derive(Debug, Clone)]
pub struct Image {
    pub path: String,
    pub alt: String,
    pub title: String,
    pub caption: Vec<ContentNode>,
    pub set: ImageSet,
    pub lqip: Option<String>,
    pub width: u32,
    pub height: u32,
    pub number: usize,
    pub gradient: Option<Vec<[u8; 4]>>,
}

#[derive(Debug, Clone, Default)]
pub struct ImageSet {
    pub webp: Vec<ImageSource>,
    pub avif: Vec<ImageSource>,
}

#[derive(Debug, Clone, Copy)]
pub enum ImageType {
    Webp,
    Avif,
}

impl Display for ImageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ext = match self {
            Self::Webp => "webp",
            Self::Avif => "avif",
        };
        write!(f, "{ext}")
    }
}

#[derive(Debug, Clone)]
pub struct ImageSourceSet {
    pub img_type: ImageType,
    pub set: Vec<ImageSource>,
}

#[derive(Debug, Clone)]
pub struct ImageSource {
    pub width: u32,
    pub path: String,
}

impl Image {
    #[cfg(feature = "ssr")]
    fn try_from_parser(
        dest_url: String,
        title: String,
        parser: &mut Parser,
        tracer: &mut PageTracer,
    ) -> Result<Self> {
        let number = tracer.img();
        let mut caption = vec![];
        while let Some(event) = parser.next() {
            if event == Event::End(TagEnd::Image) {
                break;
            }
            let node = ContentNode::try_from_event(event, parser, tracer)?;
            caption.push(node);
        }

        let source = content_dir().join("assets").join(&dest_url);
        let is_svg = source
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("svg"));
        let (path, lqip, set, width, height, gradient) = if is_svg {
            let (path, width, height) = Self::preprocess_svg(&source)?;
            (path, None, ImageSet::default(), width, height, None)
        } else {
            let (lqip, set, width, height, gradient) = Self::preprocess(&source, number)?;
            (
                format!("/assets/{dest_url}"),
                lqip,
                set,
                width,
                height,
                gradient,
            )
        };

        Ok(Self {
            path,
            alt: plain_text(&caption),
            title,
            caption,
            lqip,
            set,
            width,
            height,
            number,
            gradient,
        })
    }

    #[cfg(feature = "ssr")]
    pub fn preprocess(
        path: &Path,
        _number: usize,
    ) -> Result<(Option<String>, ImageSet, u32, u32, Option<Vec<[u8; 4]>>)> {
        use base64::{Engine as _, engine::general_purpose::STANDARD};
        use fast_image_resize::PixelType;
        use fast_image_resize::images::Image;

        let img_bytes = std::fs::read(path).map_err(|source| Error::Asset {
            path: path.to_path_buf(),
            source,
        })?;

        let img_hash = blake3::hash(&img_bytes).to_hex()[..12].to_string();

        let rgba = image::load_from_memory(&img_bytes)?.into_rgba8();
        let transparent = rgba.pixels().any(|pixel| pixel.0[3] < u8::MAX);
        let (width, height) = rgba.dimensions();
        let dim_ratio = f64::from(height) / f64::from(width);
        let img = Image::from_vec_u8(width, height, rgba.into_raw(), PixelType::U8x4)?;

        std::fs::create_dir_all(Path::new("target").join("site-assets"))?;

        let lqip_source =
            Self::preprocess_img(path, &img_hash, ImageType::Webp, &img, 16, dim_ratio, 40)?;

        let lqip_file_name = Path::new(&lqip_source.path).file_name().unwrap();
        let lqip_path = Path::new("target").join("site-assets").join(lqip_file_name);
        let lqip_mem = std::fs::read(lqip_path)?;

        let lqip_rgba = image::load_from_memory(&lqip_mem)?.into_rgba8();
        let (lqip_width, lqip_height) = lqip_rgba.dimensions();
        let y = lqip_height / 2;
        let gradient = Some(
            (0..4)
                .map(|i| {
                    let x = ((2 * i + 1) * lqip_width / 8).min(lqip_width - 1);
                    let pixel = lqip_rgba[(x, y)].0;
                    lighten(saturate(pixel, GRADIENT_SATURATION), GRADIENT_LIGHTEN)
                })
                .collect::<Vec<_>>(),
        );

        let lqip = (!transparent)
            .then(|| format!("data:image/webp;base64,{}", STANDARD.encode(&*lqip_mem)));

        let widths = if cfg!(debug_assertions) {
            vec![320]
        } else {
            vec![460, 640, 720, 920, 1280, 1920]
        };
        let mut webp_set = Vec::new();
        let mut avif_set = Vec::new();
        for w in widths {
            let img_type = ImageType::Webp;
            let img_source =
                Self::preprocess_img(path, &img_hash, img_type, &img, w, dim_ratio, 80)?;
            webp_set.push(img_source);
            let img_type = ImageType::Avif;
            let img_source =
                Self::preprocess_img(path, &img_hash, img_type, &img, w, dim_ratio, 70)?;
            avif_set.push(img_source);
        }

        let set = ImageSet {
            webp: webp_set,
            avif: avif_set,
        };

        Ok((lqip, set, width, height, gradient))
    }

    #[cfg(feature = "ssr")]
    fn preprocess_svg(path: &Path) -> Result<(String, u32, u32)> {
        let svg_error = |reason: String| Error::Svg {
            path: path.to_path_buf(),
            reason,
        };
        let file = std::fs::read(path)?;
        let hash = blake3::hash(&file).to_hex()[..12].to_string();
        let Some(file_stem) = path.file_stem() else {
            return Err(Error::InvalidMarkdown("missing file name"));
        };
        let file_name = format!("{}-{}.svg", file_stem.to_string_lossy(), hash);
        let dir = Path::new("target").join("site-assets");
        let file_path = dir.join(&file_name);

        let optimized = if file_path.exists() {
            std::fs::read_to_string(&file_path)?
        } else {
            let input = std::fs::read_to_string(path).map_err(|source| Error::Asset {
                path: path.to_path_buf(),
                source,
            })?;
            let optimized = svgm_core::optimize(&input)
                .map_err(|error| svg_error(error.to_string()))?
                .data;
            std::fs::create_dir_all(&dir)?;
            std::fs::write(&file_path, &optimized)?;
            optimized
        };
        let (width, height) = svg_dimensions(&optimized)
            .ok_or_else(|| svg_error("needs width and height or a viewBox".to_string()))?;

        Ok((format!("/assets/{}", file_name.to_string()), width, height))
    }

    #[cfg(feature = "ssr")]
    fn preprocess_img(
        path: &Path,
        hash: &str,
        img_type: ImageType,
        img: &fast_image_resize::images::Image,
        width: u32,
        dim_ratio: f64,
        quality: u8,
    ) -> Result<ImageSource> {
        use fast_image_resize::images::Image;
        use fast_image_resize::{FilterType, PixelType, ResizeAlg, ResizeOptions, Resizer};

        let Some(file_stem) = path.file_stem() else {
            return Err(Error::InvalidMarkdown("missing file name"));
        };
        let file_name = format!("{}-{}-{}w", file_stem.to_string_lossy(), hash, width,);

        let file_path = Path::new("target")
            .join("site-assets")
            .join(&file_name)
            .with_extension(img_type.to_string());

        if !file_path.exists() {
            let height = (dim_ratio * width as f64) as u32;
            let mut dst = Image::new(width, height, PixelType::U8x4);

            Resizer::new().resize(
                img,
                &mut dst,
                &ResizeOptions::new().resize_alg(ResizeAlg::Convolution(FilterType::Lanczos3)),
            )?;
            let img_mem = match img_type {
                ImageType::Webp => {
                    let encoder = webp::Encoder::from_rgba(dst.buffer(), width, height);
                    let mem: webp::WebPMemory = encoder.encode(quality as f32);
                    mem.to_vec()
                }
                ImageType::Avif => {
                    use image::ImageEncoder as _;
                    use image::codecs::avif::AvifEncoder;

                    let speed = if cfg!(debug_assertions) { 10 } else { 2 };

                    let mut mem = Vec::new();
                    AvifEncoder::new_with_speed_quality(&mut mem, speed, quality).write_image(
                        dst.buffer(),
                        width,
                        height,
                        image::ExtendedColorType::Rgba8,
                    )?;
                    mem
                }
            };

            std::fs::write(&file_path, &img_mem)?;
        }
        Ok(ImageSource {
            width,
            path: format!("/assets/{file_name}.{img_type}"),
        })
    }
}

#[cfg(feature = "ssr")]
fn svg_dimensions(svg: &str) -> Option<(u32, u32)> {
    use svgm_core::ast::NodeKind;

    let doc = svgm_core::parser::parse(svg).ok()?;
    let root = doc
        .children(doc.root)
        .find_map(|id| match &doc.node(id).kind {
            NodeKind::Element(element) if element.name == "svg" => Some(element),
            _ => None,
        })?;
    let length = |name: &str| -> Option<f64> {
        let value = root.attr(name)?.trim();
        value.strip_suffix("px").unwrap_or(value).parse().ok()
    };
    let view_box = || -> Option<(f64, f64)> {
        let values = root
            .attr("viewBox")?
            .split([' ', ','])
            .filter(|value| !value.is_empty())
            .map(|value| value.parse::<f64>().ok())
            .collect::<Option<Vec<_>>>()?;
        let [_, _, width, height] = values[..] else {
            return None;
        };
        (width > 0.0 && height > 0.0).then_some((width, height))
    };

    let (width, height) = match (length("width"), length("height")) {
        (Some(width), Some(height)) => (width, height),
        (Some(width), None) => view_box().map(|(w, h)| (width, width * h / w))?,
        (None, Some(height)) => view_box().map(|(w, h)| (height * w / h, height))?,
        (None, None) => view_box()?,
    };
    let (width, height) = (width.round(), height.round());
    (width >= 1.0 && height >= 1.0).then_some((width as u32, height as u32))
}

#[cfg(feature = "ssr")]
const GRADIENT_SATURATION: f32 = 3.;

#[cfg(feature = "ssr")]
fn saturate([r, g, b, a]: [u8; 4], factor: f32) -> [u8; 4] {
    let [r, g, b] = [r, g, b].map(f32::from);
    let luma = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    let channel = |c: f32| (luma + (c - luma) * factor).round().clamp(0.0, 255.0) as u8;
    [channel(r), channel(g), channel(b), a]
}

#[cfg(feature = "ssr")]
const GRADIENT_LIGHTEN: f32 = 0.2;

#[cfg(feature = "ssr")]
fn lighten([r, g, b, a]: [u8; 4], amount: f32) -> [u8; 4] {
    let channel = |c: u8| {
        let c = f32::from(c);
        (c + (255.0 - c) * amount).round().clamp(0.0, 255.0) as u8
    };
    [channel(r), channel(g), channel(b), a]
}

#[cfg(feature = "ssr")]
fn plain_text(nodes: &[ContentNode]) -> String {
    fn push(nodes: &[ContentNode], out: &mut String) {
        for node in nodes {
            match node {
                ContentNode::Text(Text::Raw(raw)) => out.push_str(raw),
                ContentNode::Text(Text::Styled { children, .. })
                | ContentNode::Header(Header { children, .. })
                | ContentNode::Paragraph(Paragraph { children })
                | ContentNode::BlockQuote(BlockQuote { children })
                | ContentNode::Link(Link { children, .. }) => push(children, out),
                ContentNode::Image(image) => out.push_str(&image.alt),
                ContentNode::Figure(figure) => out.push_str(&figure.image.alt),
                ContentNode::SoftBreak | ContentNode::HardBreak => out.push(' '),
                ContentNode::Table(Table {
                    head,
                    rows,
                    caption,
                    ..
                }) => {
                    push(caption, out);
                    out.push(' ');
                    for cell in head.iter().chain(rows.iter().flatten()) {
                        push(&cell.children, out);
                        out.push(' ');
                    }
                }
                ContentNode::List(List { items, .. }) => {
                    for item in items {
                        push(&item.children, out);
                        out.push(' ');
                    }
                }
                ContentNode::DefinitionList(DefinitionList { items }) => {
                    for item in items {
                        let (DefinitionItem::Title(children)
                        | DefinitionItem::Definition(children)) = item;
                        push(children, out);
                        out.push(' ');
                    }
                }
                ContentNode::Component(_) => {}
                ContentNode::CodeBlock(CodeBlock { code, .. }) => out.push_str(code),
                ContentNode::Metadata(_) => {}
            }
        }
    }

    let mut text = String::new();
    push(nodes, &mut text);
    text
}

#[cfg(feature = "ssr")]
fn html_tag(html: &str) -> Option<String> {
    let name = html.trim().strip_prefix('<')?.strip_suffix('>')?.trim();
    let bare = name.strip_prefix('/').unwrap_or(name);
    (!bare.is_empty() && bare.chars().all(|c| c.is_ascii_alphanumeric()))
        .then(|| name.to_ascii_lowercase())
}

#[derive(Debug, Clone)]
pub enum TextStyle {
    Emphasis,
    Strong,
    Strikethrough,
    Superscript,
    Subscript,
    Small,
}

#[derive(Debug, Clone)]
pub enum Text {
    Styled {
        style: TextStyle,
        children: Vec<ContentNode>,
    },
    Raw(String),
}

impl Text {
    #[cfg(feature = "ssr")]
    fn try_from_parser(
        style: Option<TextStyle>,
        text: Option<String>,
        parser: &mut Parser,
        tracer: &mut PageTracer,
    ) -> Result<Self> {
        match (style, text) {
            (None, Some(text)) => Ok(Self::Raw(text)),
            (None, None) => {
                return Err(Error::InvalidMarkdown(
                    "unexpected missing text style and content",
                ));
            }
            (Some(text_style), _) => {
                let mut children = Vec::new();
                while let Some(event) = parser.next() {
                    let is_end = match text_style {
                        TextStyle::Emphasis => event == Event::End(TagEnd::Emphasis),
                        TextStyle::Strong => event == Event::End(TagEnd::Strong),
                        TextStyle::Strikethrough => event == Event::End(TagEnd::Strikethrough),
                        TextStyle::Subscript => event == Event::End(TagEnd::Subscript),
                        TextStyle::Superscript => event == Event::End(TagEnd::Superscript),
                        TextStyle::Small => matches!(
                            &event,
                            Event::InlineHtml(html) if html_tag(html).as_deref() == Some("/small")
                        ),
                    };
                    if is_end {
                        break;
                    }
                    let node = ContentNode::try_from_event(event, parser, tracer)?;
                    children.push(node);
                }
                Ok(Self::Styled {
                    style: text_style,
                    children,
                })
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum LinkKind {
    Inline,
}

#[cfg(feature = "ssr")]
impl TryFrom<LinkType> for LinkKind {
    type Error = Error;
    fn try_from(value: LinkType) -> Result<Self> {
        let kind = match value {
            LinkType::Inline => Self::Inline,
            _ => return Err(Error::InvalidMarkdown("unsupported link type")),
        };
        Ok(kind)
    }
}

#[derive(Debug, Clone)]
pub struct Link {
    pub kind: LinkKind,
    pub children: Vec<ContentNode>,
    pub title: String,
    pub url: String,
}

impl Link {
    #[cfg(feature = "ssr")]
    fn try_from_parser(
        link_type: LinkType,
        title: String,
        dest_url: String,
        parser: &mut Parser,
        tracer: &mut PageTracer,
    ) -> Result<Self> {
        let kind = link_type.try_into()?;
        let url = dest_url;
        let mut children = Vec::new();
        while let Some(event) = parser.next() {
            if event == Event::End(TagEnd::Link) {
                break;
            }
            let node = ContentNode::try_from_event(event, parser, tracer)?;
            children.push(node);
        }
        Ok(Self {
            kind,
            children,
            title,
            url,
        })
    }
}

#[derive(Debug, Clone)]
pub struct BlockQuote {
    pub children: Vec<ContentNode>,
}

impl BlockQuote {
    #[cfg(feature = "ssr")]
    fn try_from_parser(parser: &mut Parser, tracer: &mut PageTracer) -> Result<Self> {
        let mut children = Vec::new();
        while let Some(event) = parser.next() {
            if matches!(event, Event::End(TagEnd::BlockQuote(_))) {
                break;
            }
            children.push(ContentNode::try_from_event(event, parser, tracer)?);
        }
        Ok(Self { children })
    }
}

#[derive(Debug, Clone)]
pub struct Paragraph {
    pub children: Vec<ContentNode>,
}

impl Paragraph {
    #[cfg(feature = "ssr")]
    fn try_from_parser(parser: &mut Parser, tracer: &mut PageTracer) -> Result<Self> {
        let mut children = Vec::new();
        while let Some(event) = parser.next() {
            if event == Event::End(TagEnd::Paragraph) {
                break;
            }
            let node = ContentNode::try_from_event(event, parser, tracer)?;
            children.push(node);
        }
        Ok(Self { children })
    }
}

#[derive(Debug, Clone)]
pub struct List {
    pub start: Option<u64>,
    pub items: Vec<ListItem>,
}

impl List {
    #[cfg(feature = "ssr")]
    fn try_from_parser(
        start: Option<u64>,
        parser: &mut Parser,
        tracer: &mut PageTracer,
    ) -> Result<Self> {
        let mut items = Vec::new();
        while let Some(event) = parser.next() {
            match event {
                Event::Start(Tag::Item) => items.push(ListItem::try_from_parser(parser, tracer)?),
                Event::End(TagEnd::List(_)) => break,
                _ => return Err(Error::InvalidMarkdown("list only contains items")),
            }
        }
        Ok(Self { start, items })
    }
}

#[derive(Debug, Clone)]
pub struct ListItem {
    pub task: Option<bool>,
    pub children: Vec<ContentNode>,
}

impl ListItem {
    #[cfg(feature = "ssr")]
    fn try_from_parser(parser: &mut Parser, tracer: &mut PageTracer) -> Result<Self> {
        let mut task = None;
        let mut children = Vec::new();
        while let Some(event) = parser.next() {
            match event {
                Event::End(TagEnd::Item) => break,
                Event::TaskListMarker(checked) => task = Some(checked),
                event => children.push(ContentNode::try_from_event(event, parser, tracer)?),
            }
        }
        Ok(Self { task, children })
    }
}

#[derive(Debug, Clone)]
pub struct DefinitionList {
    pub items: Vec<DefinitionItem>,
}

#[derive(Debug, Clone)]
pub enum DefinitionItem {
    Title(Vec<ContentNode>),
    Definition(Vec<ContentNode>),
}

impl DefinitionList {
    #[cfg(feature = "ssr")]
    fn try_from_parser(parser: &mut Parser, tracer: &mut PageTracer) -> Result<Self> {
        let mut items = Vec::new();
        while let Some(event) = parser.next() {
            let item =
                match event {
                    Event::Start(Tag::DefinitionListTitle) => DefinitionItem::Title(
                        Self::children(TagEnd::DefinitionListTitle, parser, tracer)?,
                    ),
                    Event::Start(Tag::DefinitionListDefinition) => DefinitionItem::Definition(
                        Self::children(TagEnd::DefinitionListDefinition, parser, tracer)?,
                    ),
                    Event::End(TagEnd::DefinitionList) => break,
                    _ => {
                        return Err(Error::InvalidMarkdown(
                            "definition list only contains titles and definitions",
                        ));
                    }
                };
            items.push(item);
        }
        Ok(Self { items })
    }

    #[cfg(feature = "ssr")]
    fn children(
        end: TagEnd,
        parser: &mut Parser,
        tracer: &mut PageTracer,
    ) -> Result<Vec<ContentNode>> {
        let mut children = Vec::new();
        while let Some(event) = parser.next() {
            if event == Event::End(end) {
                break;
            }
            children.push(ContentNode::try_from_event(event, parser, tracer)?);
        }
        Ok(children)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CellAlignment {
    None,
    Left,
    Center,
    Right,
}

#[cfg(feature = "ssr")]
impl From<Alignment> for CellAlignment {
    fn from(value: Alignment) -> Self {
        match value {
            Alignment::None => Self::None,
            Alignment::Left => Self::Left,
            Alignment::Center => Self::Center,
            Alignment::Right => Self::Right,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Table {
    pub alignments: Vec<CellAlignment>,
    pub head: Vec<TableCell>,
    pub rows: Vec<Vec<TableCell>>,
    pub number: usize,
    pub caption: Vec<ContentNode>,
}

impl Table {
    #[cfg(feature = "ssr")]
    fn try_from_parser(
        alignments: Vec<Alignment>,
        parser: &mut Parser,
        tracer: &mut PageTracer,
    ) -> Result<Self> {
        let alignments = alignments.into_iter().map(CellAlignment::from).collect();
        let mut head = Vec::new();
        let mut rows = Vec::new();
        while let Some(event) = parser.next() {
            match event {
                Event::Start(Tag::TableHead) => {
                    head = Self::cells(TagEnd::TableHead, parser, tracer)?;
                }
                Event::Start(Tag::TableRow) => {
                    rows.push(Self::cells(TagEnd::TableRow, parser, tracer)?);
                }
                Event::End(TagEnd::Table) => break,
                _ => {
                    return Err(Error::InvalidMarkdown("table contains head and rows only"));
                }
            }
        }
        Ok(Self {
            alignments,
            head,
            rows,
            number: tracer.table(),
            caption: Vec::new(),
        })
    }

    #[cfg(feature = "ssr")]
    fn cells(end: TagEnd, parser: &mut Parser, tracer: &mut PageTracer) -> Result<Vec<TableCell>> {
        let mut cells = Vec::new();
        while let Some(event) = parser.next() {
            match event {
                Event::Start(Tag::TableCell) => {
                    cells.push(TableCell::try_from_parser(parser, tracer)?);
                }
                Event::End(tag_end) if tag_end == end => break,
                _ => return Err(Error::InvalidMarkdown("table row contains cells only")),
            }
        }
        Ok(cells)
    }
}

#[derive(Debug, Clone)]
pub struct TableCell {
    pub children: Vec<ContentNode>,
}

impl TableCell {
    #[cfg(feature = "ssr")]
    fn try_from_parser(parser: &mut Parser, tracer: &mut PageTracer) -> Result<Self> {
        let mut children = Vec::new();
        while let Some(event) = parser.next() {
            if event == Event::End(TagEnd::TableCell) {
                break;
            }
            children.push(ContentNode::try_from_event(event, parser, tracer)?);
        }
        Ok(Self { children })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HeaderLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
}

impl HeaderLevel {
    #[cfg(feature = "ssr")]
    fn try_from(value: HeadingLevel) -> Result<Self> {
        let level = match value {
            HeadingLevel::H1 => Self::H1,
            HeadingLevel::H2 => Self::H2,
            HeadingLevel::H3 => Self::H3,
            HeadingLevel::H4 => Self::H4,
            HeadingLevel::H5 => Self::H5,
            _ => return Err(Error::InvalidMarkdown("header level up to 5 supported")),
        };
        Ok(level)
    }
}

#[derive(Debug, Clone)]
pub struct Header {
    pub level: HeaderLevel,
    pub children: Vec<ContentNode>,
}

impl Header {
    #[cfg(feature = "ssr")]
    fn try_from_parser(
        heading_level: HeadingLevel,
        parser: &mut Parser,
        tracer: &mut PageTracer,
    ) -> Result<Self> {
        let level = HeaderLevel::try_from(heading_level)?;
        let mut children = Vec::new();
        while let Some(event) = parser.next() {
            if event == Event::End(TagEnd::Heading(heading_level)) {
                break;
            }
            let node = ContentNode::try_from_event(event, parser, tracer)?;
            children.push(node);
        }
        Ok(Self { level, children })
    }
}

#[derive(Debug, Clone)]
pub struct MarkdownPage {
    pub metadata: Metadata,
    pub nodes: Arc<[ContentNode]>,
}

impl MarkdownPage {
    #[cfg(feature = "ssr")]
    pub fn try_from_path(path: &Path) -> Result<Self> {
        let md_input = std::fs::read_to_string(path)?;
        Self::try_from_input(&md_input)
    }

    #[cfg(feature = "ssr")]
    fn try_from_input(value: &str) -> Result<Self> {
        let options = Options::all();
        let mut parser = Parser::new_ext(&value, options);
        let mut nodes = Self::parse(&mut parser)?;
        if nodes.is_empty() {
            return Err(Error::InvalidMarkdown("empty document"));
        }
        let ContentNode::Metadata(metadata) = nodes.remove(0) else {
            return Err(Error::InvalidMarkdown(
                "first content node must be metadata",
            ));
        };
        Ok(Self {
            metadata,
            nodes: nodes.into(),
        })
    }

    #[cfg(feature = "ssr")]
    fn parse(parser: &mut Parser) -> Result<Vec<ContentNode>> {
        let mut nodes = Vec::new();
        let mut tracer = PageTracer::default();
        while let Some(event) = parser.next() {
            let node = ContentNode::try_from_event(event, parser, &mut tracer)?;
            nodes.push(node);
        }
        attach_table_captions(&mut nodes);
        Ok(nodes)
    }
}

#[cfg(feature = "ssr")]
fn attach_table_captions(nodes: &mut Vec<ContentNode>) {
    let mut i = 0;
    while i < nodes.len() {
        if matches!(nodes[i], ContentNode::Table(_)) {
            let after = nodes
                .get(i + 1)
                .and_then(table_caption_prefix)
                .map(|prefix| (i + 1, prefix));
            let before = i
                .checked_sub(1)
                .and_then(|index| Some((index, table_caption_prefix(&nodes[index])?)));

            if let Some((index, prefix)) = after.or(before) {
                if let ContentNode::Paragraph(Paragraph { mut children }) = nodes.remove(index) {
                    if let Some(ContentNode::Text(Text::Raw(text))) = children.first_mut() {
                        *text = text[prefix.len()..].trim_start().to_string();
                        if text.is_empty() {
                            children.remove(0);
                        }
                    }
                    if index < i {
                        i -= 1;
                    }
                    if let ContentNode::Table(table) = &mut nodes[i] {
                        table.caption = children;
                    }
                }
            }
        }
        i += 1;
    }
}

#[cfg(feature = "ssr")]
fn table_caption_prefix(node: &ContentNode) -> Option<&'static str> {
    let ContentNode::Paragraph(Paragraph { children }) = node else {
        return None;
    };
    let Some(ContentNode::Text(Text::Raw(text))) = children.first() else {
        return None;
    };
    ["Table:", ":"]
        .into_iter()
        .find(|prefix| text.starts_with(prefix))
}

#[cfg(feature = "ssr")]
#[derive(Debug, Default)]
pub struct PageTracer {
    img_number: usize,
    fig_number: usize,
    table_number: usize,
}

#[cfg(feature = "ssr")]
impl PageTracer {
    pub fn img(&mut self) -> usize {
        let number = self.img_number;
        self.img_number += 1;
        number
    }

    pub fn fig(&mut self) -> usize {
        let number = self.fig_number;
        self.fig_number += 1;
        number
    }

    pub fn table(&mut self) -> usize {
        let number = self.table_number;
        self.table_number += 1;
        number
    }
}
