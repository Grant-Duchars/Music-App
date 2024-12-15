use super::Icon;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn nav_bar() -> impl IntoView {
    view! {
        <nav>
            <menu>
                <li>
                    <A href="/library">
                        <Icon id="disc"/>
                        "Library"
                    </A>
                </li>
                <li>
                    <A href="/playlists">
                        <Icon id="playlist"/>
                        "Playlists"
                    </A>
                </li>
                <li>
                    <A href="/visualizer">
                        <Icon id="visualizer"/>
                        "Visualizer"
                    </A>
                </li>
            </menu>
            <menu>
                <li>
                    <A href="/settings">
                        <Icon id="settings"/>
                        "Settings"
                    </A>
                </li>
            </menu>
        </nav>
    }
}
