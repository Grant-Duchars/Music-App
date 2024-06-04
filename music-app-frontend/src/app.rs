use crate::components::{media_bar::MediaBar, nav_bar::NavBar};
use crate::library::LibraryRoute;
use leptos::*;
use leptos_router::{Redirect, Route, Router, Routes};
use leptos_use::{use_event_listener, use_window};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Copy, Clone)]
pub struct WindowWidth(pub ReadSignal<usize>);

#[component]
#[allow(unused_must_use)]
pub fn App() -> impl IntoView {
    // Setting up window width signal and event listener
    let (window_width, set_window_width) = create_signal(get_window_width());
    use_event_listener(use_window(), ev::resize, move |_| {
        set_window_width.set(get_window_width());
    });
    // Pass down the window width to the album grid component
    provide_context(WindowWidth(window_width));
    view! {
        <Router>
            <header>
                <NavBar/>
            </header>
            <main>
                <Routes>
                    <Route path="" view=|| view! { <Redirect path="library"/> }/>
                    <LibraryRoute/>
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
