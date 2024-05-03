mod app;
mod components;
mod library;

use app::App;
use leptos::*;
use music_app_lib::{get_albums, Albums};

fn main() {
    console_error_panic_hook::set_once();
    Albums::set(get_albums());
    mount_to_body(|| {
        view! { <App/> }
    })
}
