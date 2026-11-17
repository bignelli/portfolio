#[derive(Clone, Debug, PartialEq)]
pub struct SkillCategory {
    pub title_en: &'static str,
    pub title_pt: &'static str,
    pub icon: &'static str,
    pub skills: Vec<&'static str>,
}

pub fn get_skill_categories() -> Vec<SkillCategory> {
    vec![
        SkillCategory {
            title_en: "Backend & Systems",
            title_pt: "Backend & Sistemas",
            icon: "⚙️",
            skills: vec![
                "Python",
                "Java",
                "C / C++",
                "Rust",
                "FastAPI",
                "Spring Boot",
                "Hibernate",
                "SQLAlchemy",
                "Apache Kafka",
                "RESTful APIs",
                "Microservices",
            ],
        },
        SkillCategory {
            title_en: "Data Engineering & Automation",
            title_pt: "Engenharia de Dados & Automação",
            icon: "📊",
            skills: vec![
                "Apache Airflow",
                "ETL Pipelines",
                "Selenium",
                "Pandas",
                "SQL Modeling",
                "OCR & Text Extraction",
                "n8n Workflows",
                "Benner ERP Integration",
            ],
        },
        SkillCategory {
            title_en: "AI & LLM Integration",
            title_pt: "Inteligência Artificial & LLMs",
            icon: "🤖",
            skills: vec![
                "LLM Workflows",
                "Google Gemini",
                "ChatGPT / OpenAI",
                "Local LLMs (Ollama, Qwen)",
                "RAG Architecture",
                "Supabase pgvector",
                "Embeddings",
                "AI Agentic Coding",
            ],
        },
        SkillCategory {
            title_en: "DevOps & Cloud",
            title_pt: "DevOps & Nuvem",
            icon: "☁️",
            skills: vec![
                "Git & GitHub Actions",
                "Google Cloud Platform (GCP)",
                "Docker",
                "Linux Administration",
                "CI / CD Pipelines",
                "Pytest & JUnit",
                "Gradle & Maven",
            ],
        },
        SkillCategory {
            title_en: "Networks & Low-Level",
            title_pt: "Redes & Baixo Nível",
            icon: "🔌",
            skills: vec![
                "CAN Bus",
                "Automotive Networks",
                "Entropic Analysis",
                "Linux Kernel Internals",
                "JVM Bytecode",
                "Compilers & ASTs",
            ],
        },
        SkillCategory {
            title_en: "Languages & Communication",
            title_pt: "Idiomas & Comunicação",
            icon: "🌐",
            skills: vec![
                "Portuguese (Native / C2)",
                "English (Fluent / C1)",
                "Spanish (Basic)",
                "German (Learning)",
            ],
        },
    ]
}
