use crate::components::album::Album;
use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let (size, set_size) = create_signal(250);
    view! {
        <main class="container">
            <Album image="/public/SludgeLifeOST.png" name="Sludge Life" artist="Doseone" size/>
        </main>
    }
}
