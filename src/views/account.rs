//! Page de compte utilisateur.
use crate::models::{Content, User};
use crate::views::components::render_cards_or_empty;
use crate::views::layout::render_page;
use crate::views::utils::h;

pub fn render_account(user: &User, saved: &[Content], history: &[Content]) -> String {
    let body = format!(
        r#"
        <main class="account-shell">
            <section class="account-header">
                <p class="eyebrow">Compte parent</p>
                <h1>{}</h1>
                <p>Vos contenus sauvegardes et votre historique restent dans la base SQLite locale de dev.</p>
            </section>

            <section class="section-block">
                <div class="section-heading">
                    <h2>Sauvegardes</h2>
                    <p>{} contenu(x)</p>
                </div>
                <div class="card-grid two-cols">
                    {}
                </div>
            </section>

            <section class="section-block">
                <div class="section-heading">
                    <h2>Historique</h2>
                    <p>{} contenu(x) lance(s)</p>
                </div>
                <div class="card-grid two-cols">
                    {}
                </div>
            </section>
        </main>
        "#,
        h(&user.email),
        saved.len(),
        render_cards_or_empty(saved, "Aucun contenu sauvegarde pour l'instant."),
        history.len(),
        render_cards_or_empty(history, "Aucun contenu lance pour l'instant."),
    );

    render_page(
        "Compte - Ploopy",
        "Accueil",
        Some("/"),
        &Some(user.clone()),
        body,
    )
}
