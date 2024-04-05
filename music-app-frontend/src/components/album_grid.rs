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
    let num_per_row = create_memo(move |_| {
        with!(|window_width, album_width| {
            let album_with_gap = album_width + ALBUM_GRID_GAP;
            let mut num = window_width / album_with_gap;
            // Checks if window width can fit an extra album with no gap
            if *window_width > album_with_gap * num + album_width {
                num += 1;
            }
            num
        })
    });
    let num_rows = create_memo(move |_| {
        Albums::len()
            .expect("albums should be initialized")
            .div_ceil(num_per_row.get())
    });
    let (rows, set_rows) = create_signal(Vec::default());
    create_render_effect(move |_| {
        with!(|num_per_row, num_rows| {
            let mut new_rows = Vec::with_capacity(*num_rows);
            let mut album_index = 0;
            for i in 0..*num_rows {
                new_rows.push((i, Vec::with_capacity(*num_per_row)));
                for _ in 0..*num_per_row {
                    new_rows[i].1.push(Albums::get_album(album_index));
                    album_index += 1;
                }
            }
            set_rows.set(new_rows);
        })
    });
    view! {
        <div
            style=("--album-width", move || format!("{}px", album_width.get()))
            style=("--album-grid-gap", format!("{}px", ALBUM_GRID_GAP))
        >
            <For
                each=move || rows.get().into_iter()
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
