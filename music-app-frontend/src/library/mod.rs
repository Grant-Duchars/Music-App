mod album;
mod album_grid;
mod album_track_list;

use album_grid::AlbumGrid;
use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route},
    hooks::use_query,
    params::Params,
    path, MatchNestedRoutes,
};

// Context Structs
#[derive(Copy, Clone)]
struct NumPerRow(pub Memo<usize>);
#[derive(Copy, Clone)]
struct SelectedAlbum(pub RwSignal<Option<usize>>);
//

#[component(transparent)]
pub fn library_routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/library") view=|| view! { <Outlet/> }>
            <Route path=path!("") view=AlbumGrid/>
            <Route path=path!("/search") view=Search/>
        </ParentRoute>
    }
    .into_inner()
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
