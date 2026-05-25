use super::{NumPerRow, SelectedAlbum};
use leptos::prelude::*;
use music_app_lib::{
    runtime::{to_digital, to_words},
    AlbumStoreFields, Albums, AlbumsStoreFields, TrackStoreFields,
};
use reactive_stores::{AtIndex, Store};

#[component]
pub fn album_track_list(row_num: usize) -> impl IntoView {
    let NumPerRow(num_per_row) = expect_context();
    let SelectedAlbum(selected) = expect_context();
    let show = move || match *selected.read() {
        Some(s) => row_num == s / *num_per_row.read(),
        None => false,
    };
    let albums = expect_context::<Store<Albums>>().albums();
    view! {
        <Show when=show>

            {
                let album = AtIndex::new(albums, selected.read().expect("selected album"));
                let tracks = album.tracks();
                view! {
                    <div class="album-song-list" id="asl">
                        <div>
                            <div class="row space-between">
                                <h1>{move || album.title().get()}</h1>
                                <h2>{to_words(album.runtime().get())}</h2>
                            </div>
                            <div class="row space-between">
                                <h2 style="font-style: italic;">{move || album.artist().get()}</h2>
                                <h2>{move || album.genre().get()}</h2>
                            </div>
                            <ol style=(
                                "--asl-template",
                                calc_track_list_dimensions(tracks.read().len()),
                            )>
                                <For each=move || tracks key=|track| track.number().get() let:track>
                                    <li>
                                        {move || track.title().get()}
                                        <span>{to_digital(track.length().get())}</span>
                                    </li>
                                </For>

                            </ol>
                        </div>
                    </div>
                }
            }

        </Show>
    }
}

/// Evenly splits the tracks between two columns when less than 50
/// tracks and just fills the columns when equal or greater than 50
fn calc_track_list_dimensions(num: usize) -> String {
    if num < 50 {
        format!("repeat({}, min-content)", num.div_ceil(2))
    } else {
        String::from("repeat(25, min-content)")
    }
}
