use crate::components::album_grid::AlbumGrid;
use leptos::{ev::resize, *};
use leptos_use::{use_event_listener, use_window};
use music_app_lib::{get_albums, Albums};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
#[allow(unused_must_use)]
pub fn App() -> impl IntoView {
    // Setting up window width signal and event listener
    let (window_width, set_window_width) = create_signal(get_window_width());
    use_event_listener(use_window(), resize, move |_| {
        set_window_width.set(get_window_width());
    });
    // Create some dummy albums
    Albums::set(get_albums());
    // Setting up album grid styling
    let (album_width, _set_album_width) = create_signal(250);
    let selected = create_rw_signal(None);
    view! {
        <main>
            <AlbumGrid window_width album_width selected/>
        </main>
    }
}

fn get_window_width() -> usize {
    web_sys::window()
        .expect("should have a window")
        .inner_width()
        .expect("should have a width value")
        .as_f64()
        .expect("should be a number")
        .trunc() as usize
}
