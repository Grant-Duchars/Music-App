mod app;
mod components;
mod data;

use app::*;
use leptos::*;

fn main() {
    mount_to_body(|| {
        view! { <App/> }
    })
}
