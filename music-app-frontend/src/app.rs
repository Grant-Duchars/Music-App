use crate::components::{media_bar::MediaBar, nav_bar::NavBar};
use crate::library::LibraryRoutes;
use leptos::{ev, prelude::*};
use leptos_router::components::{Router, Routes};
use leptos_use::{use_event_listener, use_window};
use music_app_lib::{mocking::get_albums, Albums};
use reactive_stores::Store;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Copy, Clone)]
pub struct WindowWidth(pub ReadSignal<usize>);

#[component]
pub fn App() -> impl IntoView {
    let store = Store::new(Albums::new(get_albums()));
    provide_context(store);

    // Setting up window width signal and event listener
    let (window_width, set_window_width) = signal(get_window_width());
    let _ = use_event_listener(use_window(), ev::resize, move |_| {
        set_window_width.set(get_window_width());
    });
    provide_context(WindowWidth(window_width));

    let (_is_routing, set_is_routing) = signal(false);
    view! {
        <Router set_is_routing>
            <header>
                <NavBar/>
            </header>
            <main>
                <Routes fallback=|| "This page could not be found.">
                    <LibraryRoutes/>
                </Routes>
            </main>
            <footer>
                <MediaBar/>
            </footer>
        </Router>
    }
}

fn get_window_width() -> usize {
    web_sys::window()
        .expect("should be loaded")
        .inner_width()
        .expect("should have a value")
        .as_f64()
        .expect("should be a number")
        .trunc() as usize
}
