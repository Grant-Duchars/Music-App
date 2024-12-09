use super::{
    album::{Album, SkeletonAlbum},
    album_track_list::AlbumTrackList,
    NumPerRow, SelectedAlbum,
};
use crate::{app::WindowWidth, components::Icon};
use leptos::html;
use leptos::prelude::*;
use music_app_lib::{runtime::to_words, Albums};
use reactive_stores::Store;

const ALBUM_GRID_GAP: usize = 13;

#[component]
pub fn album_grid() -> impl IntoView {
    provide_context(SelectedAlbum(RwSignal::new(None)));

    // Setting up album grid dimension calculations
    let album_width = RwSignal::new(275);
    let WindowWidth(window_width) = expect_context();
    let num_per_row = Memo::new(move |_| {
        let window_width = *window_width.read();
        let album_width = *album_width.read();
        let album_with_gap = album_width + ALBUM_GRID_GAP;
        // How many albums with gap can fit the screen
        let mut num = window_width / album_with_gap;
        // Checks if window width can fit an extra album with no gap
        if window_width > album_with_gap * num + album_width {
            num += 1;
        }
        num
    });
    let albums = expect_context::<Store<Albums>>();
    let num_rows = Memo::new(move |_| albums.read().len().div_ceil(*num_per_row.read()));
    provide_context(NumPerRow(num_per_row));

    // Set up album grid signals and effects which update when grid dimensions change
    let rows = RwSignal::new(Vec::default());
    let _ = RenderEffect::new(move |_| {
        rows.update(|rows| {
            let mut album_index = 0;
            for i in 0..*num_rows.read() {
                let row = match rows.get_mut(i) {
                    Some(r) => r,
                    None => {
                        rows.push(Vec::with_capacity(*num_per_row.read()));
                        &mut rows[i]
                    }
                };
                row.clear();
                for _ in 0..*num_per_row.read() {
                    row.push(album_index);
                    album_index += 1;
                }
            }
            rows.truncate(*num_rows.read());
        });
    });

    view! {
        <div>
            <AlbumGridBar album_width/>
            <div
                id="album-grid"
                style=("--album-width", move || format!("{}px", album_width.get()))
                style=("--album-grid-gap", format!("{}px", ALBUM_GRID_GAP))
            >
                <For
                    each=move || rows.get().into_iter().enumerate()
                    key=|id| (id.0, id.1.len())
                    children=move |(row_num, albums)| {
                        view! {
                            <AlbumGridRow albums/>
                            <AlbumTrackList row_num/>
                        }
                    }
                />

            </div>
        </div>
    }
}

#[component]
fn album_grid_bar(album_width: RwSignal<usize>) -> impl IntoView {
    let albums = expect_context::<Store<Albums>>();
    let album_width_input = NodeRef::<html::Input>::new();
    view! {
        <div id="album-grid-bar">
            <div class="row">
                <Icon id="search" size=30 color="var(--color-light)" flipped=true/>
                <input type="search" placeholder="Search" incremental="true" class="full-width"/>
            </div>
            <div class="row">
                <p>
                    {move || to_words(albums.read().runtime())} " : "
                    {move || albums.read().total_tracks()} " tracks"
                </p>
                <input
                    node_ref=album_width_input
                    type="range"
                    min="175"
                    max="300"
                    value=album_width.get_untracked()
                    step="1"
                    on:input=move |_| {
                        let album_width_input = album_width_input.get().expect("should be loaded");
                        album_width.set(album_width_input.value_as_number() as usize);
                    }
                />

            </div>
        </div>
    }
}

#[component]
fn album_grid_row(albums: Vec<usize>) -> impl IntoView {
    let global_albums = expect_context::<Store<Albums>>().read();
    view! {
        <div class="album-grid-row">
            {albums
                .into_iter()
                .map(|num| {
                    let album = global_albums.check(num);
                    match album {
                        true => view! { <Album num/> }.into_any(),
                        false => view! { <SkeletonAlbum/> }.into_any(),
                    }
                })
                .collect_view()}
        </div>
    }
}
