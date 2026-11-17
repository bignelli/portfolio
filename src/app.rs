use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::components::navbar::Navbar;
use crate::pages::home::HomePage;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages page titles, meta tags, etc.
    provide_meta_context();

    let lang = RwSignal::new("en".to_string());
    let dark_mode = RwSignal::new(true);

    view! {
        <Title text=move || {
            if lang.get() == "en" {
                "Guilherme Bignelli // Software Engineer"
            } else {
                "Guilherme Bignelli // Engenheiro de Software"
            }
        }/>

        <div
            class="portfolio-app"
            class:theme-dark=move || dark_mode.get()
            class:theme-light=move || !dark_mode.get()
            attr:data-theme=move || if dark_mode.get() { "dark" } else { "light" }
        >
            <Navbar lang=lang dark_mode=dark_mode />
            <Router>
                <main>
                    <Routes fallback=|| "Page not found.".into_view()>
                        <Route
                            path=StaticSegment("")
                            view=move || view! { <HomePage lang=lang /> }
                        />
                    </Routes>
                </main>
            </Router>
        </div>
    }
}
