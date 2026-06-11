//! Page de la sélection scientifique.
use crate::models::{Benefit, User};
use crate::views::layout::render_page_with_class;
use crate::views::utils::h;

pub fn render_science(user: &Option<User>, benefits: &[Benefit]) -> String {
    let mut cards = String::new();
    for benefit in benefits.iter().take(4) {
        cards.push_str(&format!(
            r#"
            <article class="science-card">
                <span class="pill {}">{}</span>
                <p>{}</p>
                <small>Source : {}</small>
            </article>
            "#,
            h(benefit.key),
            h(benefit.label),
            h(benefit.detail),
            h(benefit.source),
        ));
    }

    let body = format!(
        r#"
        <section class="science-hero">
            <h1>Comment on selectionne<br>nos contenus</h1>
            <p>Chaque contenu est valide selon des criteres pediatriques scientifiquement etablis.</p>
        </section>
        <main class="science-flow">
            {}
            <div class="science-warning">
                <strong>54% des parents pensent que leur enfant est accro aux ecrans.</strong>
                <span>Certains dessins animes developpent les competences que vous cherchez a cultiver sans activite structuree.</span>
            </div>
            <a class="button button-primary full-width science-cta" href="/inscription?next=/bibliotheque">S'inscrire gratuitement</a>
            <p class="source-note centered">Sources completees: Lurie Children's Hospital 2025 · INSERM · Arcom · LeadingTree 2023</p>
        </main>
        "#,
        cards
    );

    render_page_with_class(
        "Selection scientifique - Kroissant",
        "Accueil",
        Some("/"),
        user,
        body,
        "science-page",
    )
}
