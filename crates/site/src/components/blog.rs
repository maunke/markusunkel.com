use crate::markdown::CustomComponent;
#[cfg(feature = "ssr")]
use crate::markdown::{PageTracer, Params};
use leptos::prelude::*;

#[derive(Debug)]
pub struct BlogPosts;

impl CustomComponent for BlogPosts {
    #[cfg(feature = "ssr")]
    fn try_from_params(
        params: Params,
        _body: &str,
        _tracer: &mut PageTracer,
    ) -> crate::Result<Self> {
        params.finish("blog")?;
        Ok(Self)
    }

    fn view(&self) -> AnyView {
        posts_view()
    }
}

#[cfg(feature = "ssr")]
fn posts_view() -> AnyView {
    use crate::content::Content;

    let Some(content) = use_context::<Content>() else {
        return ().into_any();
    };
    let posts = content
        .blog_posts()
        .map(|(slug, page)| {
            let metadata = &page.metadata;
            let date = metadata.date.map(|d| {
                let date = d.format("%B %-d, %Y").to_string();
                let datetime = d.format("%Y-%m-%d").to_string();
                view! { <time datetime=datetime>{date}</time> }
            });
            view! { <li>{date} <a href=format!("/blog/{slug}")>{metadata.title.clone()}</a></li> }
        })
        .collect_view();

    view! { <ul class="blog-posts">{posts}</ul> }.into_any()
}

#[cfg(not(feature = "ssr"))]
fn posts_view() -> AnyView {
    ().into_any()
}
