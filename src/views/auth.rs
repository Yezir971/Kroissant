//! Pages d'authentification.
use crate::models::{AuthMode, User};
use crate::views::layout::render_page;
use crate::views::utils::{a, h};

pub fn render_auth_page(
    mode: AuthMode,
    next: Option<String>,
    error: Option<&str>,
    user: Option<&User>,
) -> String {
    if user.is_some() {
        return render_page(
            "Kroissant",
            "Accueil",
            Some("/"),
            &user.cloned(),
            r#"<main class="centered-page"><section class="auth-card"><h1>Vous etes deja connecte.</h1><a class="button button-primary full-width" href="/compte">Ouvrir mon compte</a></section></main>"#.to_string(),
        );
    }

    let (title, subtitle, action, primary, secondary_href, secondary_text) = match mode {
        AuthMode::Register => (
            "Creez votre compte gratuit",
            "Sauvegardez vos contenus et retrouvez-les a chaque visite.",
            "/inscription",
            "Creer mon compte",
            "/connexion",
            "Se connecter",
        ),
        AuthMode::Login => (
            "Connectez-vous",
            "Reprenez la ou vous vous etiez arrete.",
            "/connexion",
            "Se connecter",
            "/inscription",
            "Creer un compte",
        ),
    };

    let next_value = next.unwrap_or_else(|| "/bibliotheque".to_string());
    let secondary = format!("{}?next={}", secondary_href, a(&next_value));
    let error_html = error
        .map(|message| format!(r#"<p class="form-error">{}</p>"#, h(message)))
        .unwrap_or_default();

    let body = format!(
        r#"
        <main class="centered-page">
            <section class="auth-card">
                <h1>{}</h1>
                <p>{}</p>
                {}
                <form method="post" action="{}" class="auth-form">
                    <input type="hidden" name="next" value="{}">
                    <label>Adresse email<input name="email" type="email" placeholder="votre@email.com" autocomplete="email" required></label>
                    <label>Mot de passe<input name="password" type="password" placeholder="••••••••" autocomplete="current-password" minlength="8" required></label>
                    <button class="button button-primary full-width" type="submit">{}</button>
                </form>
                <div class="divider">ou</div>
                <a class="button button-secondary full-width" href="{}">{}</a>
                <small>Gratuit · Aucune carte requise · Donnees securisees</small>
            </section>
        </main>
        "#,
        h(title),
        h(subtitle),
        error_html,
        action,
        a(&next_value),
        primary,
        secondary,
        secondary_text,
    );

    render_page(
        title,
        "Fiche contenu",
        Some(&next_value),
        &None::<User>,
        body,
    )
}
