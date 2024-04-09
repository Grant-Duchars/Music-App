use super::{
    album::{Album, SkeletonAlbum},
    album_song_list::AlbumSongList,
};
use leptos::*;
use music_app_lib::{Album, Albums};

const ALBUM_GRID_GAP: usize = 13;

#[component]
pub fn album_grid(
    window_width: ReadSignal<usize>,
    album_width: ReadSignal<usize>,
    selected: RwSignal<Option<usize>>,
) -> impl IntoView {
    // Set up album grid dimensions memos
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
                        <AlbumGridRow num_per_row row_num albums selected/>
                        <AlbumSongList num_per_row row_num selected/>
                    }
                }
            />

        </div>
    }
}

#[component]
fn album_grid_row(
    num_per_row: Memo<usize>,
    row_num: usize,
    albums: Vec<Option<&'static Album>>,
    selected: RwSignal<Option<usize>>,
) -> impl IntoView {
    view! {
        <div class="album-grid-row">
            {albums
                .into_iter()
                .zip(0_usize..)
                .map(|(a, i)| {
                    match a {
                        Some(album) => {
                            let num = row_num * num_per_row.get_untracked() + i;
                            view! { <Album album num selected/> }
                        }
                        None => view! { <SkeletonAlbum/> },
                    }
                })
                .collect_view()}
        </div>
    }
}
