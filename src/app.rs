use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
    components::nav_menu::*,
    routes::{about::*, donate::*, home::*, music::*},
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // content for this welcome page
        <Router>
            <NavMenu/>
            <main class="container">
                <Routes>
                    <Route path="" view=|| view! { <HomePage/> }/>
                    <Route path="music" view=|| view! { <MusicPage/> }/>
                    <Route path="donate" view=|| view! {  <DonatePage/> }/>
                    <Route path="about" view=|| view! {  <AboutPage/> }/>
                </Routes>
            </main>
        </Router>
    }
}
