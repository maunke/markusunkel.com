use crate::{
    components::{Footer, Header},
    routes::Page,
};
use leptos::prelude::*;
use leptos_meta::{HashedStylesheet, MetaTags, Title, provide_meta_context};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="icon" type="image/png" sizes="64x64" href="/media/favicon-64.png" />
                <link rel="icon" type="image/png" sizes="32x32" href="/media/favicon-32.png" />
                <link rel="icon" type="image/png" sizes="16x16" href="/media/favicon-16.png" />
                <link rel="apple-touch-icon" href="/media/apple-touch-icon.png" />
                <link
                    rel="alternate"
                    type="application/rss+xml"
                    title="Markus Unkel - Blog"
                    href="/rss.xml"
                />

                <link
                    rel="preload"
                    href="/fonts/BerkeleyMono-Light.woff2"
                    r#as="font"
                    r#type="font/woff2"
                    fetchpriority="high"
                    crossorigin="anonymous"
                />
                <link
                    rel="preload"
                    href="/fonts/unkel-serif-latin-300.woff2"
                    r#as="font"
                    r#type="font/woff2"
                    fetchpriority="high"
                    crossorigin="anonymous"
                />

                <Styles options=options.clone() />
                <MetaTags />
                <AutoReload options=options.clone() />
                <HydrationScripts options=options islands=true islands_router=true />
                <script>
                    {"{const s=document.startViewTransition?.bind(document);if(s)document.startViewTransition=u=>s(async()=>{await u();scrollTo(0,0)});else{const p=history.pushState;history.pushState=function(...a){p.apply(this,a);scrollTo(0,0)}}}"}
                </script>
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
fn Styles(options: LeptosOptions) -> impl IntoView {
    #[cfg(all(feature = "ssr", not(debug_assertions)))]
    if let Some(css) = inline_css(&options) {
        return view! { <style inner_html=css></style> }.into_any();
    }
    view! { <HashedStylesheet options id="leptos" /> }.into_any()
}

#[cfg(all(feature = "ssr", not(debug_assertions)))]
fn inline_css(options: &LeptosOptions) -> Option<&'static str> {
    use std::path::Path;
    use std::sync::OnceLock;

    static CSS: OnceLock<Option<String>> = OnceLock::new();
    CSS.get_or_init(|| {
        let pkg = Path::new(&*options.site_root).join(&*options.site_pkg_dir);
        std::fs::read_dir(pkg)
            .ok()?
            .flatten()
            .map(|entry| entry.path())
            .find(|path| {
                path.extension().is_some_and(|ext| ext == "css")
                    && path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .is_some_and(|name| name.starts_with(&*options.output_name))
            })
            .and_then(|path| std::fs::read_to_string(path).ok())
    })
    .as_deref()
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Title text="Markus Unkel" />
        <Router>
            <div class="container">
                <Header />
                <main>
                    <Routes fallback=|| "">
                        <Route path=path!("*path") view=Page />
                    </Routes>
                </main>
                <Footer />
            </div>
        </Router>
    }
}
