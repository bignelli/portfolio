use leptos::prelude::*;
use crate::data::skills::get_skill_categories;

#[component]
pub fn Skills(lang: RwSignal<String>) -> impl IntoView {
    let categories = get_skill_categories();

    view! {
        <section id="skills" class="section section-alt">
            <div class="container">
                <div class="section-lead">
                    <span class="section-index">"02 //"</span>
                    <h2 class="section-title">
                        {move || if lang.get() == "en" { "TECHNICAL ARSENAL" } else { "HABILIDADES TÉCNICAS" }}
                    </h2>
                </div>

                <div class="skills-matrix">
                    {categories.into_iter().map(|cat| {
                        let skills = cat.skills;
                        let title_en = cat.title_en;
                        let title_pt = cat.title_pt;

                        view! {
                            <div class="skill-cell">
                                <div class="skill-cell-header">
                                    <span class="cell-bracket">"❯"</span>
                                    <h3 class="skill-cell-title">
                                        {move || if lang.get() == "en" { title_en } else { title_pt }}
                                    </h3>
                                </div>
                                <div class="skill-tokens">
                                    {skills.into_iter().map(|s| {
                                        view! {
                                            <span class="skill-token">{s}</span>
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
