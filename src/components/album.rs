use crate::data::Album;
use leptos::*;

#[component]
pub fn album(
    album: &'static Album,
    num: usize,
    selected: RwSignal<Option<usize>>,
) -> impl IntoView {
    let Album {
        cover,
        title,
        artist,
        songs: _,
    } = album;
    view! {
        <div
            class="album-container"
            on:click=move |_| {
                selected.update(|s| *s = if *s == Some(num) { None } else { Some(num) });
            }
        >

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
