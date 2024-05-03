use super::{NumPerRow, SelectedAlbum};
use leptos::*;
use music_app_lib::{to_digital, to_words, Albums, Song};

#[component]
pub fn album_song_list(row_num: usize) -> impl IntoView {
    let NumPerRow(num_per_row) = use_context().expect("num_per_row context");
    let SelectedAlbum(selected) = use_context().expect("selected album context");
    let show = move || {
        with!(|num_per_row, selected| match selected {
            Some(s) => row_num == s / num_per_row,
            None => false,
        })
    };
    view! {
        <Show when=show>

            {
                let album = selected.with(|s| Albums::get_album(s.unwrap()).unwrap());
                let average_color = calc_average_color(&album.cover);
                view! {
                    <div class="album-song-list" id="asl">
                        <div style=("background-color", average_color)>
                            <div class="row">
                                <h1>{&album.title}</h1>
                                <h2>{to_words(album.runtime)}</h2>
                            </div>
                            <div class="row">
                                <h2 style="font-style: italic;">{&album.artist}</h2>
                                <h2>{&album.genre}</h2>
                            </div>
                            <SongList songs=&album.songs/>
                        </div>
                    </div>
                }
            }

        </Show>
    }
}

#[component]
fn song_list(songs: &'static [Song]) -> impl IntoView {
    view! {
        <ol style=(
            "--asl-template",
            calc_song_list_dimensions(songs.len()),
        )>

            {songs
                .iter()
                .map(|s| {
                    view! { <li>{&s.title} <span>{to_digital(s.length)}</span></li> }
                })
                .collect_view()}
        </ol>
    }
}

/// Evenly splits the songs between two columns when less than 50
/// songs and just fills the columns when equal or greater than 50
fn calc_song_list_dimensions(num: usize) -> String {
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
    // Get the html document
    let doc = web_sys::window()?.document()?;
    // Create a new img element for use in the canvas
    let img: web_sys::HtmlImageElement = doc.create_element("img").ok()?.unchecked_into();
    // Create a new canvas element for drawing the image on
    let canvas: web_sys::HtmlCanvasElement = doc.create_element("canvas").ok()?.unchecked_into();
    let ctx: web_sys::CanvasRenderingContext2d = canvas.get_context("2d").ok()??.unchecked_into();
    // Set the src for the img element to the provided image
    img.set_src(image);
    img.set_attribute("crossOrigin", "").ok()?;
    // Draw the entire image to a single pixel
    ctx.set_image_smoothing_enabled(true);
    ctx.draw_image_with_html_image_element_and_dw_and_dh(&img, 0.0, 0.0, 1.0, 1.0)
        .ok()?;
    // Get the data of the single pixel which should be the average color of the image
    let color = ctx.get_image_data(0.0, 0.0, 1.0, 1.0).ok()?.data();
    // Darken the color by 30%
    let color = palette::Srgb::new(color[0], color[1], color[2])
        .into_linear()
        .darken(0.3);
    // Format the color into a stringified CSS hex value
    let [red, green, blue]: [u8; 3] = color.into_format().into();
    Some(format!("#{red:x}{green:x}{blue:x}"))
}
