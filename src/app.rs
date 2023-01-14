use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
    components::nav_menu::*,
    routes::{about::*, contact::*, donate::*, home::*},
};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,
        // content for this welcome page
        <Router>
            <NavMenu/>
            <main class="container">
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="donate" view=|cx| view! { cx, <DonatePage/> }/>
                    <Route path="about" view=|cx| view! { cx, <AboutPage/> }/>
                    <Route path="contact" view=|cx| view! { cx, <ContactPage/> }/>
                </Routes>
            </main>
        </Router>
    }
}
