use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct PageParams {
    path: Option<String>,
}

#[component]
pub fn Page() -> impl IntoView {
    let slug = use_params::<PageParams>()
        .get_untracked()
        .ok()
        .and_then(|params| params.path)
        .map(|path| path.trim_matches('/').to_string())
        .filter(|path| !path.is_empty())
        .unwrap_or_else(|| "index".to_string());

    view! {
        <For each=move || [slug.clone()] key=|slug| slug.clone() let:slug>
            {page_view(&slug)}
        </For>
    }
}

#[cfg(feature = "ssr")]
fn page_view(slug: &str) -> AnyView {
    use crate::components::MarkdownView;
    use crate::content::Content;
    let content = expect_context::<Content>();

    let Some(page) = expect_context::<Content>().page(slug).cloned() else {
        expect_context::<leptos_axum::ResponseOptions>()
            .set_status(axum::http::StatusCode::NOT_FOUND);
        let page = content.page("404").unwrap().clone();
        return view! { <MarkdownView page canonical=canonical_url(slug) blog=false /> }.into_any();
    };
    let canonical = canonical_url(slug);
    let blog = slug.starts_with("blog/");
    view! { <MarkdownView page canonical blog /> }.into_any()
}

#[cfg(not(feature = "ssr"))]
fn page_view(_slug: &str) -> AnyView {
    ().into_any()
}

#[cfg(feature = "ssr")]
pub fn canonical_url(slug: &str) -> String {
    use crate::SITE_URL;

    match slug {
        "index" => format!("{SITE_URL}"),
        slug => format!("{SITE_URL}{slug}"),
    }
}
