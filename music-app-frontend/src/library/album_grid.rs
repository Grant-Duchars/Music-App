use super::{
    album::{Album, SkeletonAlbum},
    album_song_list::AlbumSongList,
    NumPerRow, SelectedAlbum,
};
use crate::{app::WindowWidth, components::Icon};
use leptos::*;
use music_app_lib::{runtime::to_words, Album, Albums};

const ALBUM_GRID_GAP: usize = 13;

#[component]
pub fn album_grid() -> impl IntoView {
    let WindowWidth(window_width) = use_context().expect("context provided");
    // Setting up album grid styling
    let album_width = create_rw_signal(275);
    provide_context(SelectedAlbum(create_rw_signal(None)));
    // Set up album grid dimensions memo and contexts
    let num_per_row = create_memo(move |_| {
        with!(|window_width, album_width| {
            let album_with_gap = album_width + ALBUM_GRID_GAP;
            // How many albums with gap can fit the screen
            let mut num = window_width / album_with_gap;
            // Checks if window width can fit an extra album with no gap
            if *window_width > album_with_gap * num + album_width {
                num += 1;
            }
            num
        })
    });
    provide_context(NumPerRow(num_per_row));
    let num_rows = create_memo(move |_| {
        // ⌈ len / num_per_row ⌉
        Albums::len()
            .expect("should be loaded")
            .div_ceil(num_per_row.get())
    });
    // Set up album grid signals and effects
    let rows = create_rw_signal(Vec::default());
    create_render_effect(move |_| {
        // When album grid dimensions change
        with!(|num_per_row, num_rows| {
            // Update the rows
            rows.update(|rows| {
                let mut album_index = 0;
                for i in 0..*num_rows {
                    // Get from or extend rows
                    let row = match rows.get_mut(i) {
                        Some(r) => r,
                        None => {
                            rows.push(Vec::with_capacity(*num_per_row));
                            &mut rows[i]
                        }
                    };
                    // Repopulate row with correct number of albums
                    row.clear();
                    for _ in 0..*num_per_row {
                        row.push(Albums::get_album(album_index));
                        album_index += 1;
                    }
                }
                rows.truncate(*num_rows);
            });
        })
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
                            <AlbumGridRow albums row_num/>
                            <AlbumSongList row_num/>
                        }
                    }
                />

            </div>
        </div>
    }
}

#[component]
#[allow(unused_must_use)]
fn album_grid_bar(album_width: RwSignal<usize>) -> impl IntoView {
    let album_width_input = create_node_ref::<html::Input>();
    view! {
        <div id="album-grid-bar">
            <div class="row">
                <Icon id="search" size=30 color="var(--color-light)" flipped=true/>
                <input type="search" placeholder="Search" incremental="true" class="full-width"/>
            </div>
            <div class="row">
                <p>{to_words(Albums::runtime())}" : "{Albums::total_tracks()}" tracks"</p>
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
fn album_grid_row(albums: Vec<Option<&'static Album>>, row_num: usize) -> impl IntoView {
    let NumPerRow(num_per_row) = use_context().expect("context provided");
    view! {
        <div class="album-grid-row">
            {albums
                .into_iter()
                .zip(0_usize..)
                .map(|(a, i)| {
                    match a {
                        Some(album) => {
                            let num = row_num * num_per_row.get_untracked() + i;
                            view! { <Album album num/> }
                        }
                        None => view! { <SkeletonAlbum/> },
                    }
                })
                .collect_view()}
        </div>
    }
}
