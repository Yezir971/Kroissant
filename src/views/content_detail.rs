//! Page de détail d'un contenu.
use crate::models::{Benefit, Content, User};
use crate::views::components::{render_save_panel, render_cards};
use crate::views::layout::render_page;
use crate::views::utils::{a, h};

pub fn render_content_detail(
    user: &Option<User>,
    content: &Content,
    saved: bool,
    similar: &[Content],
    benefit: &Benefit,
) -> String {
    let body = format!(
        r#"
        <section class="detail-hero">
            <img class="detail-hero-bg" src="{}" alt="">
            <div class="detail-hero-overlay"></div>
            <div class="detail-hero-content">
                <h1>{}</h1>
                <p class="eyebrow">{} · {} · {}</p>
                <p class="detail-description">{}</p>
            </div>
        </section>

        <main class="detail-layout">
            <section class="detail-actions">
                {}
                <a class="button button-primary watch-wide" href="/go/{}">Regarder sur {}</a>
            </section>

            <section class="benefit-panel">
                <h2>Ce que ce contenu developpe chez votre enfant</h2>
                <ul>
                    <li><strong>{}</strong> : {}</li>
                    <li><strong>Regulation emotionnelle</strong> : Les personnages modelisent comment nommer et gerer ses emotions.</li>
                    <li><strong>Empathie</strong> : Chaque episode met en scene le point de vue de l'autre.</li>
                </ul>
            </section>

            <p class="source-note">Sources: analyse interne Kroissant · {}</p>

            <section class="section-block similar-block">
                <div class="section-heading">
                    <h2>Contenus similaires</h2>
                </div>
                <div class="card-grid two-cols">
                    {}
                </div>
            </section>
        </main>
        "#,
        a(&content.image_url),
        h(&content.title),
        h(content.platform_label()),
        h(&content.duration),
        h(&content.age_range),
        h(&content.description),
        render_save_panel(content, user.as_ref(), saved),
        a(&content.slug),
        h(content.platform_label()),
        h(benefit.label),
        h(benefit.summary),
        h(benefit.source),
        render_cards(similar)
    );

    render_page(
        &format!("{} - Kroissant", content.title),
        "Bibliotheque",
        Some("/bibliotheque"),
        user,
        body,
    )
}
