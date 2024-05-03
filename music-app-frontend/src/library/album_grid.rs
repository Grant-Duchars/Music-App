use super::{
    album::{Album, SkeletonAlbum},
    album_song_list::AlbumSongList,
    NumPerRow, SelectedAlbum,
};
use crate::app::WindowWidth;
use leptos::*;
use music_app_lib::{Album, Albums};

const ALBUM_GRID_GAP: usize = 13;

#[component]
pub fn album_grid() -> impl IntoView {
    let WindowWidth(window_width) = use_context().expect("window width context");
    // Setting up album grid styling
    let (album_width, _set_album_width) = create_signal(250);
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
            .expect("albums should be initialized")
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
    }
}

#[component]
fn album_grid_row(albums: Vec<Option<&'static Album>>, row_num: usize) -> impl IntoView {
    let NumPerRow(num_per_row) = use_context().expect("num per row context");
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