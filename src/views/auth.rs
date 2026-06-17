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
            "Ploopy",
            "Accueil",
            Some("/"),
            &user.cloned(),
            r#"<main class="centered-page"><section class="auth-card"><h1>Vous êtes déjà connecté.</h1><a class="button button-primary full-width" href="/compte">Ouvrir mon compte</a></section></main>"#.to_string(),
        );
    }

    let (page_title, title, subtitle, action, primary, secondary_href, secondary_text) = match mode {
        AuthMode::Register => (
            "Inscription - Ploopy",
            "Bonjour, <br> Bienvenue !",
            "Creez votre compte gratuit pour sauvegarder vos contenus.",
            "/inscription",
            "S'inscrire",
            "/connexion",
            "Se connecter",
        ),
        AuthMode::Login => (
            "Connexion - Ploopy",
            "Bonjour, <br> Ravis de vous revoir !",
            "",
            "/connexion",
            "Se connecter",
            "/inscription",
            "S'inscrire",
        ),
    };

    let next_value = next.unwrap_or_else(|| "/bibliotheque".to_string());
    let secondary = format!("{}?next={}", secondary_href, a(&next_value));
    let error_html = error
        .map(|message| format!(r#"<p class="form-error">{}</p>"#, h(message)))
        .unwrap_or_default();

    let confirm_html = if matches!(mode, AuthMode::Register) {
        r#"<label>Confirmation du mot de passe
            <div class="password-wrapper">
                <input id="confirm-password-input" name="confirm_password" type="password" placeholder="••••••••" minlength="8" required>
                <button type="button" class="password-toggle" onclick="togglePassword('confirm-password-input', 'confirm-password-icon')">
                    <img id="confirm-password-icon" src="/static/img/oeil.svg" alt="Afficher le mot de passe">
                </button>
            </div>
        </label>"#.to_string()
    } else {
        "".to_string()
    };

    let subtitle_html = if subtitle.is_empty() {
        "".to_string()
    } else {
        format!("<p>{}</p>", h(subtitle))
    };

    let body = format!(
        r##"
        <main class="centered-page">
            <section class="auth-card">
                <h1>{}</h1>
                {}
                {}
                <form method="post" action="{}" class="auth-form">
                    <input type="hidden" name="next" value="{}">
                    <label>E-mail<input name="email" type="email" placeholder="exemple@exemple" autocomplete="email" required></label>
                    <label>Mot de passe
                        <div class="password-wrapper">
                            <input id="password-input" name="password" type="password" placeholder="" autocomplete="current-password" minlength="8" required>
                            <button type="button" class="password-toggle" onclick="togglePassword('password-input', 'password-icon')">
                                <img id="password-icon" src="/static/img/oeil.svg" alt="Afficher le mot de passe">
                            </button>
                        </div>
                    </label>
                    <a href="#" class="forgot-password">Mot de passe oublié ?</a>
                    <label class="remember-me">
                        <input type="checkbox" name="remember_me"> <span>Se souvenir de moi</span>
                    </label>
                    {}
                    <div class="auth-actions">
                        <button class="button button-primary full-width" type="submit">{}</button>
                        <a class="button button-secondary full-width" href="{}">{}</a>
                        <a class="button button-google full-width" href="/auth/google">
                            <img src="https://www.gstatic.com/images/branding/product/1x/gsa_512dp.png" alt="Google Logo">
                            Se connecter avec Google
                        </a>
                    </div>
                </form>
            </section>
        </main>
        <script>
            function togglePassword(inputId, iconId) {{
                const input = document.getElementById(inputId);
                const icon = document.getElementById(iconId);
                if (input.type === 'password') {{
                    input.type = 'text';
                    icon.src = '/static/img/oeil-dash.svg';
                }} else {{
                    input.type = 'password';
                    icon.src = '/static/img/oeil.svg';
                }}
            }}
        </script>
        "##,
        title, 
        subtitle_html,
        error_html,
        action,
        a(&next_value),
        confirm_html,
        primary,
        secondary,
        secondary_text,
    );

    render_page(
        page_title,
        "Fiche contenu",
        Some(&next_value),
        &None::<User>,
        body,
    )
}
