use leptos::*;
use music_app_lib::{to_digital, to_words, Albums, Song};
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
                <div>

                    {
                        let album = selected.with(|s| Albums::get_album(s.unwrap()).unwrap());
                        view! {
                            <div class="album-song-list-row">
                                <h1>{&album.title}</h1>
                                <h2>{to_words(album.runtime)}</h2>
                            </div>
                            <div class="album-song-list-row">
                                <h2 style="font-style: italic;">{&album.artist}</h2>
                                <h2>{&album.genre}</h2>
                            </div>
                            <SongList songs=&album.songs/>
                        }
                    }

                </div>
            </div>
        </Show>
    }
}

#[component]
fn song_list(songs: &'static Rc<[Song]>) -> impl IntoView {
    view! {
        <ol
            class="album-song-list-list"
            style=("--asl-template", calc_song_list_dimensions(songs.len()))
        >
            {songs
                .iter()
                .map(|s: &Song| {
                    view! { <li>{&s.title} <span>{to_digital(s.length)}</span></li> }
                })
                .collect_view()}
        </ol>
    }
}

fn calc_song_list_dimensions(num: usize) -> String {
    if num < 50 {
        format!("repeat({}, min-content)", num.div_ceil(2))
    } else {
        String::from("repeat(25, min-content)")
    }
}
