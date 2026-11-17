use leptos::prelude::*;

#[component]
pub fn Contact(lang: RwSignal<String>) -> impl IntoView {
    view! {
        <section id="contact" class="section">
            <div class="container">
                <div class="section-lead">
                    <span class="section-index">"05 //"</span>
                    <h2 class="section-title">
                        {move || if lang.get() == "en" { "CONTACT // DISPATCH" } else { "CONTATO // CONEXÃO" }}
                    </h2>
                </div>

                <div class="contact-console">
                    <div class="console-prompt">
                        <span class="console-cmd">"❯ net.connect --target=guilherme.bignelli"</span>
                    </div>

                    <p class="console-desc">
                        {move || if lang.get() == "en" {
                            "Available for backend engineering roles, high-throughput data infrastructure, and systems software. Reach out directly through any of the channels below."
                        } else {
                            "Disponível para posições de engenharia backend, infraestrutura de dados e software de sistemas. Entre em contato diretamente através dos canais abaixo."
                        }}
                    </p>

                    <div class="contact-matrix">
                        <a href="mailto:guilhermebignelli@gmail.com" class="matrix-cell">
                            <span class="cell-label">"EMAIL"</span>
                            <span class="cell-value">"guilhermebignelli@gmail.com"</span>
                            <span class="cell-arrow">"↗"</span>
                        </a>

                        <a
                            href="https://www.linkedin.com/in/guilherme-bignelli-9b145920b/"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="matrix-cell"
                        >
                            <span class="cell-label">"LINKEDIN"</span>
                            <span class="cell-value">"linkedin.com/in/guilherme-bignelli"</span>
                            <span class="cell-arrow">"↗"</span>
                        </a>

                        <a
                            href="https://github.com/bignelli"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="matrix-cell"
                        >
                            <span class="cell-label">"GITHUB"</span>
                            <span class="cell-value">"github.com/bignelli"</span>
                            <span class="cell-arrow">"↗"</span>
                        </a>

                        <a href="tel:+5541987406606" class="matrix-cell">
                            <span class="cell-label">"PHONE"</span>
                            <span class="cell-value">"+55 (41) 98740-6606"</span>
                            <span class="cell-arrow">"↗"</span>
                        </a>
                    </div>
                </div>
            </div>
        </section>

        <footer class="footer">
            <div class="container footer-layout">
                <div class="footer-left">
                    <span class="footer-copyright">"© 2026 GUILHERME BIGNELLI"</span>
                    <span class="footer-divider">"//"</span>
                    <span class="footer-loc">"CURITIBA, BRAZIL"</span>
                </div>
                <div class="footer-right">
                    <span class="footer-tech">"COMPILED IN RUST + LEPTOS 0.8 CSR // WASM"</span>
                </div>
            </div>
        </footer>
    }
}
