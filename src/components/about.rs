use leptos::prelude::*;

#[component]
pub fn About(lang: RwSignal<String>) -> impl IntoView {
    view! {
        <section id="about" class="section">
            <div class="container">
                <div class="section-lead">
                    <span class="section-index">"01 //"</span>
                    <h2 class="section-title">
                        {move || if lang.get() == "en" { "ABOUT" } else { "SOBRE" }}
                    </h2>
                </div>

                <div class="about-grid">
                    <div class="about-prose">
                        <p class="about-lead-para">
                            {move || if lang.get() == "en" {
                                "I am a backend-focused software engineer based in Curitiba, Brazil. My engineering work centers on building reliable backend microservices, robust API endpoints, automated scraping and ETL pipelines, and integrating modern LLMs into real-world systems."
                            } else {
                                "Sou um engenheiro de software focado em backend em Curitiba, Brasil. Meu trabalho concentra-se no desenvolvimento de microsserviços confiáveis, APIs de alto desempenho, pipelines automatizados de scraping e ETL, e integração de LLMs a sistemas de produção."
                            }}
                        </p>

                        <p>
                            {move || if lang.get() == "en" {
                                "My background connects low-level systems engineering with cloud automation. I have researched automotive network protocols (CAN bus entropic analysis with ZF Friedrichshafen), completed Linux Kernel Foundation training (LFD103), built compiler frontends with custom ASTs in C, and authored open-source Minecraft modifications with thousands of active players using JVM bytecode instrumentation."
                            } else {
                                "Minha formação conecta engenharia de baixo nível a automações em nuvem. Pesquisei protocolos de redes automotivas (análise entrópica de barramento CAN com a ZF Friedrichshafen), concluí formação em Linux Kernel Internals (LFD103), construí frontends de compiladores com ASTs em C e criei modificações open-source para Minecraft com milhares de jogadores ativos via instrumentação JVM."
                            }}
                        </p>

                        <p>
                            {move || if lang.get() == "en" {
                                "In production environments, I prioritize clean architectural boundaries, automated testing (Pytest, JUnit), strict transactional consistency, and data security standards such as LGPD."
                            } else {
                                "Em ambientes de produção, priorizo separação arquitetural clara, testes automatizados (Pytest, JUnit), consistência transacional e padrões de segurança de dados em conformidade com a LGPD."
                            }}
                        </p>
                    </div>

                    <div class="about-details">
                        <div class="detail-panel">
                            <div class="panel-header">
                                <span class="panel-icon">"⌁"</span>
                                <span class="panel-title">
                                    {move || if lang.get() == "en" { "EDUCATION & TRAINING" } else { "FORMAÇÃO & CERTIFICAÇÕES" }}
                                </span>
                            </div>
                            <div class="panel-content">
                                <div class="edu-entry">
                                    <div class="edu-degree">"B.S. in Software Engineering"</div>
                                    <div class="edu-inst">"Universidade Católica de Brasília (UCB)"</div>
                                    <div class="edu-time">"2024 — 2027"</div>
                                </div>
                                <div class="edu-entry">
                                    <div class="edu-degree">"B.S. in Computer Engineering"</div>
                                    <div class="edu-inst">"Universidade Tecnológica Federal do Paraná (UTFPR)"</div>
                                    <div class="edu-time">"2018 — 2025 (Incomplete)"</div>
                                </div>
                                <div class="edu-entry">
                                    <div class="edu-degree">"LFD103: Linux Kernel Internals"</div>
                                    <div class="edu-inst">"The Linux Foundation"</div>
                                </div>
                            </div>
                        </div>

                        <div class="detail-panel">
                            <div class="panel-header">
                                <span class="panel-icon">"🌐"</span>
                                <span class="panel-title">
                                    {move || if lang.get() == "en" { "SPOKEN LANGUAGES" } else { "IDIOMAS" }}
                                </span>
                            </div>
                            <div class="panel-content">
                                <div class="lang-row">
                                    <span class="lang-name">
                                        {move || if lang.get() == "en" { "Portuguese" } else { "Português" }}
                                    </span>
                                    <span class="lang-level">
                                        {move || if lang.get() == "en" { "Native (C2)" } else { "Nativo (C2)" }}
                                    </span>
                                </div>
                                <div class="lang-row">
                                    <span class="lang-name">
                                        {move || if lang.get() == "en" { "English" } else { "Inglês" }}
                                    </span>
                                    <span class="lang-level">
                                        {move || if lang.get() == "en" { "Fluent (C1)" } else { "Fluente (C1)" }}
                                    </span>
                                </div>
                                <div class="lang-row">
                                    <span class="lang-name">
                                        {move || if lang.get() == "en" { "Spanish" } else { "Espanhol" }}
                                    </span>
                                    <span class="lang-level">
                                        {move || if lang.get() == "en" { "Basic" } else { "Básico" }}
                                    </span>
                                </div>
                                <div class="lang-row">
                                    <span class="lang-name">
                                        {move || if lang.get() == "en" { "German" } else { "Alemão" }}
                                    </span>
                                    <span class="lang-level">
                                        {move || if lang.get() == "en" { "Elementary (Learning)" } else { "Básico (Estudando)" }}
                                    </span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
