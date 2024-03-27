use leptos::*;

#[derive(Clone)]
pub struct AlbumData {
    cover: String,
    title: String,
    artist: String,
}

impl AlbumData {
    pub fn new(cover: String, title: String, artist: String) -> Self {
        AlbumData {
            cover,
            title,
            artist,
        }
    }
}

#[component]
fn skeleton_album() -> impl IntoView {
    view! { <div class="album-container"></div> }
}

#[component]
fn album(
    coords: RwSignal<(isize, isize)>,
    self_coords: (isize, isize),
    album: AlbumData,
) -> impl IntoView {
    let AlbumData {
        cover,
        title,
        artist,
    } = album;
    view! {
        <div
            class="album-container"
            on:click=move |_| {
                coords
                    .update(|c| {
                        if *c == self_coords {
                            *c = (-1, -1);
                        } else {
                            *c = self_coords;
                        }
                    });
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
pub fn album_list(
    albums: ReadSignal<Vec<AlbumData>>,
    num_albums: ReadSignal<usize>,
    width: ReadSignal<usize>,
    album_width: ReadSignal<usize>,
) -> impl IntoView {
    let num_per_row = create_memo(move |_| {
        with!(|width, album_width| {
            let num = width / (album_width + 13);
            if *width > (album_width + 13) * num + album_width {
                num + 1
            } else {
                num
            }
        })
    });
    let num_rows = create_memo(move |_| num_albums.get().div_ceil(num_per_row.get() as usize));
    let (rows, set_rows) = create_signal(Vec::with_capacity(0));
    create_render_effect(move |_| {
        let albums = albums.get();
        let mut rows = Vec::with_capacity(num_rows.get());
        let mut album_index = 0;
        for i in 0..num_rows.get() {
            rows.push((i, Vec::with_capacity(num_per_row.get())));
            for _ in 0..num_per_row.get() {
                let album = albums.get(album_index);
                match album {
                    Some(a) => rows[i].1.push(Some(a.clone())),
                    None => rows[i].1.push(None),
                }
                album_index += 1;
            }
        }
        set_rows.set(rows);
    });
    let coords = create_rw_signal((-1, -1));
    view! {
        <div style=("--album-width", format!("{}px", album_width.get()))>
            <For
                each=move || rows.get().into_iter()
                key=|id| (id.0, id.1.len())
                children=move |(row_num, albums)| {
                    view! { <AlbumListRow coords row_num albums/> }
                }
            />

        </div>
    }
}

#[component]
fn album_list_row(
    coords: RwSignal<(isize, isize)>,
    row_num: usize,
    albums: Vec<Option<AlbumData>>,
) -> impl IntoView {
    view! {
        <div class="album-list-row">
            {albums
                .into_iter()
                .zip(0_isize..)
                .map(|(a, i)| {
                    match a {
                        Some(album) => {
                            let self_coords = (row_num as isize, i);
                            view! { <Album coords self_coords album/> }
                        }
                        None => view! { <SkeletonAlbum/> },
                    }
                })
                .collect_view()}
        </div>
        <Show when=move || coords.with(|(x, _)| *x == row_num as isize) fallback=|| view! {}>
            <AlbumSongList coords/>
        </Show>
    }
}

#[component]
fn album_song_list(coords: RwSignal<(isize, isize)>) -> impl IntoView {
    view! {
        <div class="album-song-list">
            <h1>"Coords: " {coords}</h1>
        </div>
    }
}
