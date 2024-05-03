pub mod media_bar;
pub mod nav_bar;

use leptos::*;

#[component]
pub fn icon(id: &'static str) -> impl IntoView {
    view! {
        <svg class="icon">
            <use_ href=format!("public/icons.svg#{}", id)></use_>
        </svg>
    }
}
