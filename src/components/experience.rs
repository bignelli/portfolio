use leptos::prelude::*;

struct ExperienceItem {
    company: &'static str,
    role_en: &'static str,
    role_pt: &'static str,
    period_en: &'static str,
    period_pt: &'static str,
    location_en: &'static str,
    location_pt: &'static str,
    bullets_en: Vec<&'static str>,
    bullets_pt: Vec<&'static str>,
    tags: Vec<&'static str>,
}

#[component]
pub fn Experience(lang: RwSignal<String>) -> impl IntoView {
    let experiences = vec![
        ExperienceItem {
            company: "Technology Institute of Paraná (Tecpar)",
            role_en: "Backend Software & IT Intern",
            role_pt: "Estagiário de Software Backend & TI",
            period_en: "Mar 2026 — Present",
            period_pt: "Mar 2026 — Presente",
            location_en: "Curitiba, Brazil",
            location_pt: "Curitiba, Brasil",
            tags: vec!["Python", "FastAPI", "Selenium", "Qwen / Ollama", "pgvector", "n8n"],
            bullets_en: vec![
                "Engineered automated Python data scraping pipelines using Selenium to monitor and extract unstructured data from public bidding documents.",
                "Built and exposed high-throughput microservice APIs using FastAPI.",
                "Integrated local LLMs (Qwen via Ollama) into backend pipelines for automated text extraction and regulatory analysis from complex PDFs.",
                "Architected a fertilizer cross-validation system utilizing autonomous AI agents and Supabase pgvector.",
                "Automated corporate ERP workflows (Benner) via n8n and assisted with switch/network infrastructure monitoring.",
            ],
            bullets_pt: vec![
                "Desenvolveu pipelines de extração automatizada de dados em Python com Selenium para monitoramento de editais de licitações públicas.",
                "Construiu e expôs APIs de microsserviços de alto desempenho utilizando FastAPI.",
                "Integrou LLMs locais (Qwen via Ollama) ao backend para extração e análise textual automatizada de PDFs regulatórios complexos.",
                "Arquitetou uma aplicação de validação cruzada regulatória de fertilizantes com agentes de IA e Supabase pgvector.",
                "Automatizou fluxos de ERP corporativo (Benner) via n8n e participou do monitoramento de infraestrutura de rede e switches.",
            ],
        },
        ExperienceItem {
            company: "Fall Engenharia",
            role_en: "Junior Software Engineer",
            role_pt: "Engenheiro de Software Júnior",
            period_en: "Feb 2025 — Mar 2026",
            period_pt: "Fev 2025 — Mar 2026",
            location_en: "Remote",
            location_pt: "Remoto",
            tags: vec!["Java", "Spring Boot", "Apache Kafka", "Hibernate", "PostgreSQL", "JUnit"],
            bullets_en: vec![
                "Developed modular, scalable backend services in Java using Spring Boot and Hibernate ORM.",
                "Implemented and monitored fault-tolerant asynchronous messaging pipelines with Apache Kafka.",
                "Designed and versioned secure, high-performance RESTful APIs following microservices standards.",
                "Maintained rigorous testing suites with JUnit and Spring Test within a remote agile Kanban methodology.",
            ],
            bullets_pt: vec![
                "Desenvolveu serviços modulares de backend em Java utilizando Spring Boot e Hibernate ORM.",
                "Implementou e monitorou fluxos de mensageria assíncrona orientados a eventos com Apache Kafka.",
                "Projetou e versionou APIs RESTful seguras e de alta performance seguindo padrões de microsserviços.",
                "Manteve suítes rigorosas de testes unitários e de integração com JUnit e Spring Test em ambiente ágil remoto.",
            ],
        },
        ExperienceItem {
            company: "Solvefin",
            role_en: "Software Engineer",
            role_pt: "Engenheiro de Software",
            period_en: "May 2024 — Jan 2025",
            period_pt: "Mai 2024 — Jan 2025",
            location_en: "Remote",
            location_pt: "Remoto",
            tags: vec!["Python", "Airflow", "GCP", "SQLAlchemy", "Selenium", "OpenAI API", "Docker"],
            bullets_en: vec![
                "Built and deployed CI/CD pipelines using GitHub Actions, Apache Airflow DAGs, and Google Cloud Platform.",
                "Developed Python web scrapers and ETL workflows using Selenium, Pandas, and SQLAlchemy.",
                "Connected backend pipelines with Gemini & OpenAI APIs, combining OCR with LLMs for unstructured document analysis.",
                "Enforced strict LGPD compliance for financial data handling across microservices.",
            ],
            bullets_pt: vec![
                "Construiu e implantou pipelines de CI/CD com GitHub Actions, DAGs no Apache Airflow e Google Cloud Platform.",
                "Desenvolveu raspadores web e fluxos ETL em Python utilizando Selenium, Pandas e SQLAlchemy.",
                "Conectou serviços backend a APIs da OpenAI e Gemini, combinando OCR com LLMs para processamento documental.",
                "Garantia de conformidade estrita com a LGPD no tratamento de dados financeiros sensíveis.",
            ],
        },
        ExperienceItem {
            company: "ZF Friedrichshafen / UTFPR",
            role_en: "Research Intern in Automotive Networks",
            role_pt: "Estagiário de Pesquisa em Redes Automotivas",
            period_en: "Mar 2021 — Jun 2021",
            period_pt: "Mar 2021 — Jun 2021",
            location_en: "Curitiba, Brazil",
            location_pt: "Curitiba, Brasil",
            tags: vec!["Python", "C++", "CAN Bus", "Linux", "GitLab CI"],
            bullets_en: vec![
                "Engineered an automotive network emulator and simulator in Python and C++.",
                "Conducted entropic data analysis on real-time CAN bus traffic streams to detect protocol anomalies.",
                "Provided technical support for Linux embedded environments and configured automated testing in GitLab CI/CD.",
            ],
            bullets_pt: vec![
                "Desenvolveu um emulador e simulador de redes automotivas em Python e C++.",
                "Conduziu análise entrópica em fluxos de dados de barramento CAN em tempo real para identificação de anomalias.",
                "Suporte técnico em ambientes Linux embarcados e configuração de rotinas automatizadas no GitLab CI/CD.",
            ],
        },
    ];

    view! {
        <section id="experience" class="section">
            <div class="container">
                <div class="section-lead">
                    <span class="section-index">"03 //"</span>
                    <h2 class="section-title">
                        {move || if lang.get() == "en" { "WORK EXPERIENCE" } else { "EXPERIÊNCIA PROFISSIONAL" }}
                    </h2>
                </div>

                <div class="log-timeline">
                    {experiences.into_iter().map(|item| {
                        let company = item.company;
                        let role_en = item.role_en;
                        let role_pt = item.role_pt;
                        let period_en = item.period_en;
                        let period_pt = item.period_pt;
                        let location_en = item.location_en;
                        let location_pt = item.location_pt;
                        let bullets_en = item.bullets_en;
                        let bullets_pt = item.bullets_pt;
                        let tags = item.tags;

                        view! {
                            <div class="log-card">
                                <div class="log-meta-bar">
                                    <span class="log-period">
                                        {move || if lang.get() == "en" { period_en } else { period_pt }}
                                    </span>
                                    <span class="log-loc">
                                        {move || if lang.get() == "en" { location_en } else { location_pt }}
                                    </span>
                                </div>

                                <div class="log-header">
                                    <h3 class="log-company">{company}</h3>
                                    <div class="log-role">
                                        {move || if lang.get() == "en" { role_en } else { role_pt }}
                                    </div>
                                </div>

                                <ul class="log-bullets">
                                    {move || {
                                        if lang.get() == "en" {
                                            bullets_en.clone().into_iter().map(|b| view! {
                                                <li>
                                                    <span class="bullet-caret">"❯"</span>
                                                    <span>{b}</span>
                                                </li>
                                            }).collect_view()
                                        } else {
                                            bullets_pt.clone().into_iter().map(|b| view! {
                                                <li>
                                                    <span class="bullet-caret">"❯"</span>
                                                    <span>{b}</span>
                                                </li>
                                            }).collect_view()
                                        }
                                    }}
                                </ul>

                                <div class="log-tags">
                                    {tags.into_iter().map(|t| view! {
                                        <span class="log-tag">{t}</span>
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
