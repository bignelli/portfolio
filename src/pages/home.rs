use leptos::prelude::*;
use crate::components::{
    about::About,
    contact::Contact,
    experience::Experience,
    hero::Hero,
    projects::Projects,
    skills::Skills,
};

#[component]
pub fn HomePage(lang: RwSignal<String>) -> impl IntoView {
    view! {
        <div class="home-page">
            <Hero lang=lang />
            <About lang=lang />
            <Skills lang=lang />
            <Projects lang=lang />
            <Experience lang=lang />
            <Contact lang=lang />
        </div>
    }
}
