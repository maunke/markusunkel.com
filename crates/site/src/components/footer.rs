use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <div>
                <A href={"/"}>"© 2026 Markus Unkel"</A>
            </div>
            <div class="footer-links">
                <A href={"/legal"}>Legal</A>
                <A href={"/privacy"}>Privacy</A>
                <a href="/rss.xml" rel="external">
                    RSS
                </a>
            </div>
        </footer>
    }
}
