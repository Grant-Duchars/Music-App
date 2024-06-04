use super::SelectedAlbum;
use leptos::*;
use music_app_lib::Album;

// test
#[component]
pub fn album(album: &'static Album, num: usize) -> impl IntoView {
    let SelectedAlbum(selected) = use_context().expect("context provided");
    let Album {
        cover,
        title,
        artist,
        songs: _,
        genre: _,
        runtime: _,
    } = album;
    view! {
        <div
            class="album-container"
            on:click=move |_| {
                selected.update(|s| *s = if *s == Some(num) { None } else { Some(num) });
            }
        >

            // selected
            // .with(|s| {
            // if s.is_some() {
            // let doc = web_sys::window()
            // .expect("should be loaded")
            // .document()
            // .expect("should be loaded");
            // doc.get_element_by_id("asl")
            // .expect("should be loaded")
            // .scroll_into_view_with_bool(false);
            // }
            // });

            <div class="album-cover-image">
                <img src=cover/>
            </div>
            <h1>{title}</h1>
            <h2>{artist}</h2>
        </div>
    }
}

#[component]
pub fn skeleton_album() -> impl IntoView {
    view! { <div class="album-container"></div> }
}
