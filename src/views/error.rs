//! Pages d'erreur.
use crate::views::utils::h;

pub fn render_error_page(message: &str) -> String {
    format!(
        r#"<!doctype html>
        <html lang="fr">
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <title>Kroissant</title>
            <link rel="stylesheet" href="/static/app.css">
        </head>
        <body>
            <main class="centered-page">
                <section class="auth-card">
                    <h1>{}</h1>
                    <a class="button button-primary full-width" href="/">Retour a l'accueil</a>
                </section>
            </main>
        </body>
        </html>"#,
        h(message)
    )
}
