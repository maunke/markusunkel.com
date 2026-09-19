use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header>
            <nav>
                <div style="flex-grow:1;">
                    <A href={"/"}>Markus Unkel</A>
                </div>
                <div style="display: flex; gap: 20px;">
                    <A href="/freelance">Freelance</A>
                    <A href="/about">About</A>
                </div>
            </nav>
        </header>
    }
}
