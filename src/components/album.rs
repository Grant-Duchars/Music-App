use leptos::*;

pub struct AlbumData {
    pub cover: String,
    pub title: String,
    pub artist: String,
}

#[component]
pub fn album(image: String, name: String, artist: String, size: ReadSignal<u32>) -> impl IntoView {
    view! {
        <div class="album-container" style=("--album-width", format!("{}px", size.get()))>
            <AlbumCoverImage image/>
            <h1>{name}</h1>
            <h2>{artist}</h2>
        </div>
    }
}

#[component]
pub fn album_cover_image(image: String) -> impl IntoView {
    view! {
        <div class="album-cover-image">
            <img src=image/>
        </div>
    }
}

#[component]
pub fn album_list(albums: Vec<AlbumData>, size: ReadSignal<u32>) -> impl IntoView {
    view! {
        <div class="album-grid">
            {albums
                .into_iter()
                .map(|a| {
                    view! { <Album image=a.cover name=a.title artist=a.artist size/> }
                })
                .collect_view()}
        </div>
    }
}
