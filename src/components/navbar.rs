use leptos::prelude::*;

#[component]
pub fn Navbar(
    lang: RwSignal<String>,
    dark_mode: RwSignal<bool>,
) -> impl IntoView {
    let menu_open = RwSignal::new(false);

    let toggle_theme = move |_| {
        dark_mode.update(|d| *d = !*d);
    };

    let toggle_lang = move |_| {
        lang.update(|l| {
            if l == "en" {
                *l = "pt".to_string();
            } else {
                *l = "en".to_string();
            }
        });
    };

    let close_menu = move |_| {
        menu_open.set(false);
    };

    view! {
        <nav class="navbar">
            <div class="navbar-container">
                <a href="#hero" class="nav-brand" on:click=close_menu>
                    <span class="nav-brand-bracket">"["</span>
                    <span class="nav-brand-code">"gb"</span>
                    <span class="nav-brand-bracket">"]"</span>
                    <span class="nav-brand-name">"guilherme.bignelli"</span>
                </a>

                <ul class="nav-links" class:active=move || menu_open.get()>
                    <li>
                        <a href="#about" class="nav-link" on:click=close_menu>
                            <span class="nav-num">"01."</span>
                            <span>{move || if lang.get() == "en" { "about" } else { "sobre" }}</span>
                        </a>
                    </li>
                    <li>
                        <a href="#skills" class="nav-link" on:click=close_menu>
                            <span class="nav-num">"02."</span>
                            <span>{move || if lang.get() == "en" { "skills" } else { "skills" }}</span>
                        </a>
                    </li>
                    <li>
                        <a href="#experience" class="nav-link" on:click=close_menu>
                            <span class="nav-num">"03."</span>
                            <span>{move || if lang.get() == "en" { "experience" } else { "experiência" }}</span>
                        </a>
                    </li>
                    <li>
                        <a href="#projects" class="nav-link" on:click=close_menu>
                            <span class="nav-num">"04."</span>
                            <span>{move || if lang.get() == "en" { "projects" } else { "projetos" }}</span>
                        </a>
                    </li>
                    <li>
                        <a href="#contact" class="nav-link" on:click=close_menu>
                            <span class="nav-num">"05."</span>
                            <span>{move || if lang.get() == "en" { "contact" } else { "contato" }}</span>
                        </a>
                    </li>
                </ul>

                <div class="nav-controls">
                    <button class="control-btn" on:click=toggle_lang aria-label="Toggle language">
                        <span class="control-prefix">"LANG:"</span>
                        <span class="control-val">{move || if lang.get() == "en" { "PT" } else { "EN" }}</span>
                    </button>
                    <button class="control-btn" on:click=toggle_theme aria-label="Toggle theme">
                        <span class="control-prefix">"MODE:"</span>
                        <span class="control-val">{move || if dark_mode.get() { "LIGHT" } else { "DARK" }}</span>
                    </button>
                    <button
                        class="nav-hamburger"
                        on:click=move |_| menu_open.update(|open| *open = !*open)
                        aria-label="Toggle navigation menu"
                    >
                        <span class="hamburger-line"></span>
                        <span class="hamburger-line"></span>
                    </button>
                </div>
            </div>
        </nav>
    }
}
