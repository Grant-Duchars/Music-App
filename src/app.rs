use crate::components::album::{AlbumData, AlbumList};
use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let (size, _set_size) = create_signal(250);
    let mut albums = Vec::with_capacity(10);
    for _ in 0..10 {
        albums.push(AlbumData {
            cover: "/public/SludgeLifeOST.png".into(),
            title: "Sludge Life".into(),
            artist: "Doseone".into(),
        })
    }
    view! {
        <main>
            <AlbumList albums size/>
        </main>
    }
}
