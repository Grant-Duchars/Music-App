use super::SelectedAlbum;
use leptos::prelude::*;
use music_app_lib::{AlbumStoreFields, Albums, AlbumsStoreFields};
use reactive_stores::{AtIndex, Store};

#[component]
pub fn album(num: usize) -> impl IntoView {
    let SelectedAlbum(selected) = expect_context();
    let albums = expect_context::<Store<Albums>>().albums();
    let album = AtIndex::new(albums, num);
    view! {
        <div
            class="album-container"
            on:click=move |_| {
                *selected.write() = if *selected.read() == Some(num) { None } else { Some(num) };
            }
        >

            <div class="album-cover-image">
                <img src=move || album.cover().get()/>
            </div>
            <h1>{move || album.title().get()}</h1>
            <h2>{move || album.artist().get()}</h2>
        </div>
    }
}

#[component]
pub fn skeleton_album() -> impl IntoView {
    view! { <div class="album-container"></div> }
}
