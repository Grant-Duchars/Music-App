use crate::data::{Albums, Song};
use leptos::*;
use std::rc::Rc;

#[component]
pub fn album_song_list(
    num_per_row: Memo<usize>,
    row_num: usize,
    selected: RwSignal<Option<usize>>,
) -> impl IntoView {
    view! {
        <Show
            when=move || {
                with!(
                    | num_per_row, selected | { if let Some(n) = selected { row_num == n /
                    num_per_row } else { false } }
                )
            }

            fallback=|| view! {}
        >

            <div class="album-song-list">

                {
                    let album = selected.with(|s| Albums::get_album(s.unwrap()).unwrap());
                    view! {
                        <h1>{&album.title}</h1>
                        <h2>{&album.artist}</h2>
                        <SongList songs=&album.songs/>
                    }
                }

            </div>
        </Show>
    }
}

#[component]
fn song_list(songs: &'static Rc<[Song]>) -> impl IntoView {
    view! {
        <ul>
            {songs
                .iter()
                .map(|s: &Song| {
                    view! {
                        <li>
                            <p>{s.number} {&s.title}</p>
                        </li>
                    }
                })
                .collect_view()}
        </ul>
    }
}
