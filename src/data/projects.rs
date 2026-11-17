#[derive(Clone, Debug, PartialEq)]
pub struct ProjectData {
    pub id: &'static str,
    pub title_en: &'static str,
    pub title_pt: &'static str,
    pub description_en: &'static str,
    pub description_pt: &'static str,
    pub technologies: Vec<&'static str>,
    pub github_url: Option<&'static str>,
    pub live_url: Option<&'static str>,
}

pub fn get_projects() -> Vec<ProjectData> {
    vec![
        ProjectData {
            id: "portfolio-csr",
            title_en: "Rust + Leptos Static CSR Portfolio",
            title_pt: "Portfólio Estático CSR em Rust + Leptos",
            description_en: "Client-side rendered static portfolio website engineered with Rust, Leptos 0.8, WebAssembly, and custom plain CSS. Zero JavaScript framework dependencies, instant dark/light theming, and bilingual localization.",
            description_pt: "Website de portfólio pessoal estático com renderização client-side (CSR) desenvolvido em Rust, Leptos 0.8, WebAssembly e CSS puro. Sem frameworks JS, com troca instantânea de tema claro/escuro e localização bilíngue.",
            technologies: vec!["Rust", "Leptos 0.8", "WASM", "CSR", "Trunk", "CSS3"],
            github_url: Some("https://github.com/bignelli/portfolio"),
            live_url: None,
        },
        ProjectData {
            id: "kregbot",
            title_en: "KregBot: Java Chat Interpreter",
            title_pt: "KregBot: Interpretador Java em Chat",
            description_en: "Java-powered Discord bot capable of compiling and interpreting Java code snippets directly through chat. Features custom RPG command mechanics and relational persistence backed by the Spring Framework.",
            description_pt: "Bot para Discord em Java capaz de compilar e interpretar trechos de código Java submetidos diretamente pelo chat. Conta com mecânicas de RPG e persistência relacional com Spring Framework.",
            technologies: vec!["Java", "Spring Boot", "Discord API", "PostgreSQL", "JVM"],
            github_url: Some("https://github.com/FarraindesLokoes/KregBot"),
            live_url: None,
        },
        ProjectData {
            id: "minecraft-mods",
            title_en: "Open-Source Minecraft Mods",
            title_pt: "Mods Open-Source para Minecraft",
            description_en: "Published and actively maintained modifications with thousands of downloads. Implemented low-level JVM bytecode instrumentation, Forge API hooks, and custom algorithmic data structures for real-time game performance.",
            description_pt: "Modificações publicadas e mantidas com milhares de downloads. Implementação de instrumentação de bytecode JVM em baixo nível, hooks na Forge API e estruturas de dados algorítmicas para desempenho em tempo real.",
            technologies: vec!["Java", "JVM Bytecode", "Forge API", "Gradle", "ASM"],
            github_url: Some("https://github.com/bignelli"),
            live_url: None,
        },
    ]
}
