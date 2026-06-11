//! Modèle représentant un bénéfice développemental.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Benefit {
    pub key: &'static str,
    pub label: &'static str,
    pub summary: &'static str,
    pub detail: &'static str,
    pub source: &'static str,
}

impl Benefit {
    pub fn all() -> Vec<Benefit> {
        vec![
            Benefit {
                key: "resilience",
                label: "Resilience",
                summary: "Apprendre a surmonter les obstacles en maintenant l'effort.",
                detail: "Capacite a surmonter les obstacles. Modelisee par les personnages via l'echec, la perseverance et la recuperation emotionnelle.",
                source: "Analyse 150 episodes Bluey / Tandfonline, 2025",
            },
            Benefit {
                key: "empathie",
                label: "Empathie",
                summary: "Comprendre le point de vue de l'autre dans des situations simples.",
                detail: "Comprehension du point de vue de l'autre. Les contenus mettent en scene des situations ou le personnage doit se mettre a la place d'autrui.",
                source: "Prithviraj et al., 2024 / Impact of cartoons on childhood development",
            },
            Benefit {
                key: "language",
                label: "Developpement du langage",
                summary: "Nommer, raconter et reformuler ce qui vient d'etre vu.",
                detail: "Vocabulaire, narration, comprehension orale. Les programmes educatifs favorisent activement ces dimensions.",
                source: "Cohorte Elfe / INSERM, 2023 / Etude longitudinale sur 14 000 enfants francais",
            },
            Benefit {
                key: "regulation",
                label: "Regulation emotionnelle",
                summary: "Reconnaître ses emotions et choisir une reponse adaptee.",
                detail: "Nommer, reconnaître et gerer ses emotions. Reduit la reactivite emotionnelle a long terme.",
                source: "JAMA Pediatrics - Radesky et al., 2023",
            },
            Benefit {
                key: "creativite",
                label: "Creativite",
                summary: "Explorer plusieurs idees sans chercher une seule bonne reponse.",
                detail: "Les recits ouverts encouragent les enfants a inventer, comparer et tester des variantes.",
                source: "LeadingTree 2023 - Early creative learning review",
            },
            Benefit {
                key: "science",
                label: "Science",
                summary: "Observer, poser une question et verifier une hypothese simple.",
                detail: "Les contenus scientifiques de qualite structurent la curiosite: observation, prediction, verification et vocabulaire precis.",
                source: "Arcom 2024 - Jeunesse et contenus educatifs",
            },
        ]
    }

    pub fn for_skill(skill: &str) -> Benefit {
        Self::all()
            .into_iter()
            .find(|b| b.key == skill)
            .unwrap_or(Benefit {
                key: "resilience",
                label: "Resilience",
                summary: "Apprendre a surmonter les obstacles en maintenant l'effort.",
                detail: "Capacite a surmonter les obstacles.",
                source: "Analyse Kroissant",
            })
    }
}
