mod album;
mod album_grid;
mod album_song_list;

use album_grid::AlbumGrid;
use leptos::*;
use leptos_router::*;

#[derive(Copy, Clone)]
struct NumPerRow(pub Memo<usize>);

#[derive(Copy, Clone)]
struct SelectedAlbum(pub RwSignal<Option<usize>>);

#[component(transparent)]
pub fn library_route() -> impl IntoView {
    view! {
        <Route path="library" view=|| view! { <Outlet/> }>
            <Route path="" view=AlbumGrid/>
            <Route path="search" view=Search/>
        </Route>
    }
}

#[derive(Params, PartialEq, Clone, Debug)]
struct Search {
    q: Option<String>,
}

#[component]
pub fn search() -> impl IntoView {
    let query = use_query::<Search>();
    view! {
        <div>
            <h1>"Library Search"</h1>
            <h2>"Query: " {format!("{:?}", query.get())}</h2>
        </div>
    }
}
