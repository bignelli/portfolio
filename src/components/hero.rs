use leptos::prelude::*;

#[component]
pub fn Hero(lang: RwSignal<String>) -> impl IntoView {
    view! {
        <section id="hero" class="hero-section">
            <div class="container hero-grid">
                <div class="hero-text">
                    <div class="terminal-prompt">
                        <span class="prompt-symbol">"❯"</span>
                        <span class="prompt-cmd">"sys.locate"</span>
                        <span class="prompt-separator">"//"</span>
                        <span class="prompt-loc">"Curitiba, PR, Brazil"</span>
                        <span class="status-indicator">
                            <span class="status-dot"></span>
                            <span class="status-text">
                                {move || if lang.get() == "en" { "READY" } else { "DISPONÍVEL" }}
                            </span>
                        </span>
                    </div>

                    <h1 class="hero-title">
                        "Guilherme"
                        <br/>
                        <span class="hero-surname">"Bignelli"</span>
                    </h1>

                    <h2 class="hero-subtitle">
                        {move || if lang.get() == "en" {
                            "Backend Software Engineer"
                        } else {
                            "Engenheiro de Software Backend"
                        }}
                    </h2>

                    <p class="hero-tagline">
                        {move || if lang.get() == "en" {
                            "Focused on high-performance backend services, automated data extraction pipelines, and integrating local/cloud LLMs into resilient architectures. Hands-on in Python, Java, C/C++, and Rust."
                        } else {
                            "Focado em serviços de backend de alto desempenho, pipelines automatizados de extração de dados e integração de LLMs locais e em nuvem a arquiteturas resilientes. Experiência prática em Python, Java, C/C++ e Rust."
                        }}
                    </p>

                    <div class="terminal-spec-box">
                        <div class="spec-row">
                            <span class="spec-label">"CORE"</span>
                            <span class="spec-dots">"····"</span>
                            <span class="spec-value">"Rust · Python · Java · C/C++"</span>
                        </div>
                        <div class="spec-row">
                            <span class="spec-label">"DATA"</span>
                            <span class="spec-dots">"····"</span>
                            <span class="spec-value">"Airflow · Selenium · Pandas · SQL"</span>
                        </div>
                        <div class="spec-row">
                            <span class="spec-label">"SYSTEMS"</span>
                            <span class="spec-dots">"·"</span>
                            <span class="spec-value">"Linux Internals · CAN Bus · Kafka · GCP"</span>
                        </div>
                    </div>

                    <div class="hero-actions">
                        <a href="#contact" class="btn btn-primary">
                            <span>{move || if lang.get() == "en" { "Contact" } else { "Contato" }}</span>
                            <span class="btn-arrow">"→"</span>
                        </a>
                        <a
                            href="https://github.com/bignelli"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="btn btn-secondary"
                        >
                            <span>"GitHub"</span>
                            <span class="btn-arrow">"↗"</span>
                        </a>
                        <a
                            href="https://www.linkedin.com/in/guilherme-bignelli-9b145920b/"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="btn btn-secondary"
                        >
                            <span>"LinkedIn"</span>
                            <span class="btn-arrow">"↗"</span>
                        </a>
                    </div>
                </div>

                <div class="hero-frame-wrapper">
                    <div class="photo-frame">
                        <div class="frame-corner top-left">"+"</div>
                        <div class="frame-corner top-right">"+"</div>
                        <div class="frame-corner bottom-left">"+"</div>
                        <div class="frame-corner bottom-right">"+"</div>
                        <div class="frame-header">
                            <span class="frame-id">"IMG_REF: GB_DEV"</span>
                            <span class="frame-status">"ACTIVE"</span>
                        </div>
                        <!-- Replace src with your actual photo at /images/photo.jpg anytime -->
                        <div class="frame-img-container">
                            <img
                                src="/images/avatar-placeholder.svg"
                                alt="Guilherme Bignelli"
                                class="hero-photo"
                                width="300"
                                height="300"
                            />
                        </div>
                        <div class="frame-footer">
                            <span class="frame-coords">"LAT -25.4284 // LON -49.2733"</span>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
