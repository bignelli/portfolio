use leptos::prelude::*;
use crate::data::projects::get_projects;

#[component]
pub fn Projects(lang: RwSignal<String>) -> impl IntoView {
    let projects = get_projects();

    view! {
        <section id="projects" class="section section-alt">
            <div class="container">
                <div class="section-lead">
                    <span class="section-index">"04 //"</span>
                    <h2 class="section-title">
                        {move || if lang.get() == "en" { "FEATURED BUILDS" } else { "PROJETOS EM DESTAQUE" }}
                    </h2>
                </div>

                <div class="builds-grid">
                    {projects.into_iter().enumerate().map(|(idx, proj)| {
                        let id_str = format!("PRJ_{:02}", idx + 1);
                        let title_en = proj.title_en;
                        let title_pt = proj.title_pt;
                        let desc_en = proj.description_en;
                        let desc_pt = proj.description_pt;
                        let techs = proj.technologies;
                        let github = proj.github_url;

                        view! {
                            <div class="build-card">
                                <div class="build-header">
                                    <span class="build-tag">{id_str}</span>
                                    {github.map(|url| {
                                        view! {
                                            <a
                                                href=url
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                class="build-link-btn"
                                            >
                                                <span>"SOURCE"</span>
                                                <span class="link-arrow">"↗"</span>
                                            </a>
                                        }
                                    })}
                                </div>

                                <h3 class="build-title">
                                    {move || if lang.get() == "en" { title_en } else { title_pt }}
                                </h3>

                                <p class="build-description">
                                    {move || if lang.get() == "en" { desc_en } else { desc_pt }}
                                </p>

                                <div class="build-tech-list">
                                    {techs.into_iter().map(|t| {
                                        view! {
                                            <span class="build-tech-pill">{t}</span>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}
