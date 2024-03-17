use leptos::*;

#[component]
pub fn album(
    #[prop(into)] image: String,
    #[prop(into)] name: String,
    #[prop(into)] artist: String,
    size: ReadSignal<u32>,
) -> impl IntoView {
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
