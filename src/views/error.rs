//! Pages d'erreur.
use crate::models::User;
use crate::views::layout::render_page;
use crate::views::utils::h;

pub fn render_404(user: &Option<User>) -> String {
    let body = format!(
        r#"
        <main class="centered-page">
            <section class="auth-card">
                <div class="centered">
                    <span style="font-size: 4rem;">404</span>
                    <h1>Oh non ! Tu t'es perdu ?</h1>
                    <p style="font-size: 1.2rem; margin: 20px 0;">
                        Ne t'inquiète pas, petit explorateur ! <br>
                        Le chemin que tu cherches a disparu comme par magie... 
                        Peut-être qu'un petit lutin l'a caché ?
                    </p>
                    <a class="button button-primary" href="/" style="width: auto; padding: 0 40px;">
                        Vite, retournons à la maison !
                    </a>
                </div>
            </section>
        </main>
        "#
    );

    render_page("Page introuvable - Ploopy", "Erreur 404", None, user, body)
}

pub fn render_error_page(message: &str) -> String {
    format!(
        r#"<!doctype html>
        <html lang="fr">
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <title>Ploopy</title>
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
