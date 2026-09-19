use crate::Result;
use crate::markdown::MarkdownPage;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PageCollection {
    pub pages: HashMap<String, Arc<MarkdownPage>>,
}

#[derive(Debug, Clone)]
pub struct Content {
    pub root: Arc<PageCollection>,
    pub blog: Arc<PageCollection>,
    blog_order: Arc<[(String, Arc<MarkdownPage>)]>,
}

static CONTENT_DIR: &'static str = "content";

fn content_dir() -> &'static Path {
    Path::new(CONTENT_DIR)
}

impl Content {
    pub fn page(&self, slug: &str) -> Option<&MarkdownPage> {
        let page = match slug.split_once('/') {
            Some(("blog", slug)) => self.blog.pages.get(slug),
            Some(_) => None,
            None => self.root.pages.get(slug),
        };
        page.map(Arc::as_ref)
    }

    pub fn blog_posts(&self) -> impl Iterator<Item = (&str, &MarkdownPage)> {
        self.blog_order
            .iter()
            .map(|(slug, page)| (slug.as_str(), page.as_ref()))
    }

    pub fn load() -> Result<Self> {
        let root = Arc::new(Self::load_collection("")?);
        let blog = Arc::new(Self::load_collection("blog")?);
        let mut blog_order = blog
            .pages
            .iter()
            .map(|(slug, page)| (slug.clone(), Arc::clone(page)))
            .collect::<Vec<_>>();
        blog_order.sort_by(|(a_slug, a), (b_slug, b)| {
            b.metadata
                .date
                .cmp(&a.metadata.date)
                .then_with(|| a_slug.cmp(b_slug))
        });
        Ok(Self {
            root,
            blog,
            blog_order: blog_order.into(),
        })
    }

    fn load_collection(path: &str) -> Result<PageCollection> {
        let dir = content_dir().join(path);
        let mut pages = HashMap::new();
        for entry in std::fs::read_dir(dir)? {
            let entry_path = entry?.path();
            if entry_path.extension().is_some_and(|ext| ext == "md") {
                let page = MarkdownPage::try_from_path(&entry_path)?;

                if let Some(slug) = entry_path.file_stem().and_then(|stem| stem.to_str()) {
                    pages.insert(slug.to_string(), Arc::new(page));
                }
            }
        }
        Ok(PageCollection { pages })
    }
}
