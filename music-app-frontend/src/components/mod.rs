pub mod media_bar;
pub mod nav_bar;

use leptos::prelude::*;

#[component]
pub fn icon(
    id: &'static str,
    #[prop(optional)] size: usize,
    #[prop(default = "var(--color-light-dim)")] color: &'static str,
    #[prop(optional)] flipped: bool,
) -> impl IntoView {
    view! {
        <div
            class="icon"
            style=format!(
                "fill:{};{}",
                color,
                match size {
                    0 => String::new(),
                    size => format!("height:{size}px;width:{size}px;"),
                },
            )
        >
            <svg style=match flipped {
                false => "",
                true => "transform:scale(-1,1)",
            }>
                <use_ href=format!("public/icons.svg#{}", id)></use_>
            </svg>
        </div>
    }
}
