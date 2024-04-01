use crate::{
    components::album_grid::AlbumGrid,
    data::{Album, Albums, Song},
};
use leptos::{ev::resize, *};
use leptos_use::{use_event_listener, use_window};
use std::rc::Rc;
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

/// Function to mock pulling music data from backend
fn get_albums() -> Vec<Album> {
    let artist = "Doseone";
    let mut titles = vec![
        "Boot Up",
        "Glug Sucks",
        "Climbing Pipe",
        "Mosca Mosca Mosca",
        "Nothing Works",
        "Island Fever",
        "Tia Knows Best",
        "All Ducks Everything",
        "Hi Frog Eye",
        "B Boys Revenge",
        "Ll Cool Hans",
        "Bass Water",
        "Sedative Express",
        "Double Double",
        "The Whistler",
        "Big Mud Dont Sleep",
        "Ghost Phone Home",
        "Uzzi Does It",
        "Dryer Trier",
        "Zoomed Out",
        "Burgermon",
        "Toxic Environment",
        "Eyes Out",
        "Ghost Whip the Ceo Ship",
        "Life Loop Blues",
        "Big Mud - Bubble Up",
        "Big Mud - Sludge Life",
    ]
    .into_iter();
    let mut songs = Vec::new();
    for i in 1_usize..=27 {
        songs.push(Song::new(titles.next().unwrap(), artist, i))
    }
    let songs: Rc<[Song]> = Rc::from(songs);
    let mut albums = Vec::with_capacity(10);
    for _ in 0..10 {
        albums.push(Album::new(
            "/public/SludgeLifeOST.png",
            "Sludge Life",
            "Doseone",
            songs.clone(),
        ));
    }
    albums
}
