use super::Icon;
use leptos::*;
use leptos_use::use_event_listener;

#[component]
#[allow(unused_must_use)]
pub fn media_bar() -> impl IntoView {
    let audio = create_node_ref::<html::Audio>();
    view! {
        <audio
            node_ref=audio
            src="https://upload.wikimedia.org/wikipedia/commons/transcoded/b/b1/Haussperling2008.ogg/Haussperling2008.ogg.mp3"
        ></audio>
        <div id="now-playing-image">
            <img src="public/mock-covers/sl.jpg"/>
        </div>
        <div id="media-bar">
            <LeftMediaControls audio/>
            <SeekBar audio/>
            <RightMediaControls audio/>
        </div>
    }
}

#[component]
#[allow(unused_must_use)]
fn left_media_controls(audio: NodeRef<html::Audio>) -> impl IntoView {
    let playing = create_rw_signal(false);
    use_event_listener(audio, ev::ended, move |_| {
        playing.set(false);
    });
    view! {
        <div class="row">
            <button>
                <Icon id="media-back"/>
            </button>
            <button on:click=move |_| {
                let audio = audio.get().expect("should be loaded");
                if audio.paused() {
                    let _ = audio.play();
                    playing.set(true);
                } else {
                    let _ = audio.pause();
                    playing.set(false);
                }
            }>
                <Show when=move || playing.get() fallback=|| view! { <Icon id="media-play"/> }>
                    <Icon id="media-pause"/>
                </Show>
            </button>
            <button>
                <Icon id="media-forward"/>
            </button>
        </div>
    }
}

#[component]
#[allow(unused_must_use)]
fn right_media_controls(audio: NodeRef<html::Audio>) -> impl IntoView {
    let _ = audio;
    let volume_bar = create_node_ref::<html::Button>();
    let volume_bar_hover = create_rw_signal(false);
    view! {
        <div class="row">
            <Show
                when=move || !volume_bar_hover.get()
                fallback=move || {
                    view! {
                        // Volume Bar
                        <button
                            node_ref=volume_bar
                            style="width:175px;"
                            on:mouseleave=move |_| {
                                volume_bar_hover.set(false);
                            }
                        >

                            <Icon id="speaker"/>
                            <input
                                type="range"
                                max="100"
                                step="1"
                                style="width:130px;margin-right:7px;"
                            />
                        </button>
                    }
                }
            >

                // Right Media Controls
                <button
                    node_ref=volume_bar
                    on:mouseenter=move |_| {
                        volume_bar_hover.set(true);
                    }
                >

                    <Icon id="speaker"/>
                </button>
                <button>
                    <Icon id="media-shuffle"/>
                </button>
                <button>
                    <Icon id="media-repeat"/>
                </button>
                <button>
                    <Icon id="circle-star"/>
                </button>
                <button>
                    <Icon id="circle-menu"/>
                </button>
            </Show>
        </div>
    }
}

#[component]
#[allow(unused_must_use)]
fn seek_bar(audio: NodeRef<html::Audio>) -> impl IntoView {
    let seek_bar = create_node_ref::<html::Input>();
    let seek_bar_mouse_down = create_rw_signal(false);
    let track_num = create_rw_signal(Some("1"));
    let track_title = create_rw_signal(Some("Boot Up"));
    let track_artist = create_rw_signal(Some("Doseone"));
    // Event Listeners
    use_event_listener(audio, ev::timeupdate, move |_| {
        if !seek_bar_mouse_down.get_untracked() {
            let audio = audio.get().expect("should be loaded");
            let seek_bar = seek_bar.get().expect("should be loaded");
            seek_bar.set_value_as_number(audio.current_time() / audio.duration() * 100.0);
        }
    });
    use_event_listener(audio, ev::loadeddata, move |_| {
        let seek_bar = seek_bar.get().expect("should be loaded");
        seek_bar.set_value_as_number(0.0);
    });
    view! {
        <div id="seek-bar">
            <div>
                <div>
                    <Show when=move || track_num.get().is_some()>
                        <p style="color:var(--color-light-dim);">{track_num} ": "</p>
                    </Show>
                    <Show
                        when=move || track_title.get().is_some()
                        fallback=|| view! { <p>"Nothing Playing"</p> }
                    >
                        <p>{track_title}</p>
                    </Show>
                    <Show when=move || track_artist.get().is_some()>
                        <p style="font-style:italic;color:var(--color-light-dim);">
                            " - " {track_artist}
                        </p>
                    </Show>
                </div>
                <div>
                    <p>{music_app_lib::runtime::to_digital(10)}</p>
                </div>
            </div>
            <input
                node_ref=seek_bar
                type="range"
                max="100"
                step="1"
                on:mousedown=move |_| {
                    seek_bar_mouse_down.set(true);
                }

                on:mouseup=move |_| {
                    seek_bar_mouse_down.set(false);
                }

                on:change=move |_| {
                    let audio = audio.get().expect("should be loaded");
                    let seek_bar = seek_bar.get().expect("should be loaded");
                    let percent = seek_bar.value_as_number() / 100.0;
                    audio.set_current_time(audio.duration() * percent);
                }
            />

        </div>
    }
}
