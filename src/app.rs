use crate::components::album::{AlbumData, AlbumList};
use leptos::{ev::resize, html::Main, *};
use leptos_use::{use_event_listener, use_window};
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
    let (width, set_width) = create_signal(
        web_sys::window()
            .expect("should have a window")
            .inner_width()
            .expect("should have a width value")
            .as_f64()
            .expect("should be a number") as usize
            - 10,
    );
    let main = create_node_ref::<Main>();
    use_event_listener(use_window(), resize, move |_| {
        set_width.set(
            main.get()
                .expect("main should be loaded")
                .get_bounding_client_rect()
                .width() as usize,
        );
    });
    // Create some dummy albums
    let mut albums = Vec::with_capacity(10);
    for _ in 0..10 {
        albums.push(AlbumData::new(
            "/public/SludgeLifeOST.png".into(),
            "Sludge Life".into(),
            "Doseone".into(),
        ));
    }
    let (num_albums, _set_num_albums) = create_signal(albums.len());
    let (albums, _set_albums) = create_signal(albums);
    // Setting up album grid styling
    let (album_width, _set_album_width) = create_signal(250);
    view! {
        <main _ref=main>
            <AlbumList albums num_albums width album_width/>
        </main>
    }
}
