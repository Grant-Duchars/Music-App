use super::{NumPerRow, SelectedAlbum};
use leptos::prelude::*;
use music_app_lib::{
    runtime::{to_digital, to_words},
    AlbumStoreFields, Albums, AlbumsStoreFields, TrackStoreFields,
};
use reactive_stores::{AtIndex, Store};

#[component]
pub fn album_track_list(row_num: usize) -> impl IntoView {
    let NumPerRow(num_per_row) = expect_context();
    let SelectedAlbum(selected) = expect_context();
    let show = move || match *selected.read() {
        Some(s) => row_num == s / *num_per_row.read(),
        None => false,
    };
    let albums = expect_context::<Store<Albums>>().albums();
    view! {
        <Show when=show>

            {
                let album = AtIndex::new(albums, selected.read().expect("selected album"));
                let tracks = album.tracks();
                let average_color = calc_average_color(&album.cover().read());
                view! {
                    <div class="album-song-list" id="asl">
                        <div style=("background-color", average_color)>
                            <div class="row space-between">
                                <h1>{move || album.title().get()}</h1>
                                <h2>{to_words(album.runtime().get())}</h2>
                            </div>
                            <div class="row space-between">
                                <h2 style="font-style: italic;">{move || album.artist().get()}</h2>
                                <h2>{move || album.genre().get()}</h2>
                            </div>
                            <ol style=(
                                "--asl-template",
                                calc_track_list_dimensions(tracks.read().len()),
                            )>
                                <For each=move || tracks key=|track| track.number().get() let:track>
                                    <li>
                                        {move || track.title().get()}
                                        <span>{to_digital(track.length().get())}</span>
                                    </li>
                                </For>

                            </ol>
                        </div>
                    </div>
                }
            }

        </Show>
    }
}

/// Evenly splits the tracks between two columns when less than 50
/// tracks and just fills the columns when equal or greater than 50
fn calc_track_list_dimensions(num: usize) -> String {
    if num < 50 {
        format!("repeat({}, min-content)", num.div_ceil(2))
    } else {
        String::from("repeat(25, min-content)")
    }
}

fn calc_average_color(image: &str) -> String {
    calc_average_color_helper(image).unwrap_or(String::from("#000000"))
}

fn calc_average_color_helper(image: &str) -> Option<String> {
    use palette::Darken;
    use wasm_bindgen::JsCast;
    use web_sys::*;
    // Get the html document
    let doc = window()?.document()?;
    // Create a new img element for use in the canvas
    let img: HtmlImageElement = doc.create_element("img").ok()?.unchecked_into();
    // Create a new canvas element for drawing the image on
    let canvas: HtmlCanvasElement = doc.create_element("canvas").ok()?.unchecked_into();
    let ctx: CanvasRenderingContext2d = canvas.get_context("2d").ok()??.unchecked_into();
    // Set the src for the img element to the provided image
    img.set_src(image);
    img.set_attribute("crossOrigin", "").ok()?;
    // Draw the entire image to a single pixel
    ctx.set_image_smoothing_enabled(true);
    ctx.draw_image_with_html_image_element_and_dw_and_dh(&img, 0.0, 0.0, 1.0, 1.0)
        .ok()?;
    // Get the data of the single pixel which should be the average color of the image
    let color = ctx.get_image_data(0.0, 0.0, 1.0, 1.0).ok()?.data();
    // Darken the color by 10%
    let color = palette::Srgb::new(color[0], color[1], color[2])
        .into_linear()
        .darken(0.1);
    // Format the color into a stringified CSS hex value
    let [red, green, blue]: [u8; 3] = color.into_format().into();
    Some(format!("#{red:x}{green:x}{blue:x}"))
}
