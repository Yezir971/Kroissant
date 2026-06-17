//! Vue pour la confirmation de création de compte.
use crate::views::utils::h;

pub fn render(pseudo: &str) -> String {
    format!(
        r##"
        <div style="padding: 40px; text-align: center;">
            <div style="margin-bottom: 32px;">
                <h1 style="font-family: var(--font-title); font-size: 2rem; margin-bottom: 8px;">Bravo !</h1>
                <p style="color: var(--muted); font-size: 1.1rem;">J'ai hâte de voyager dans ces univers avec vous, {}.</p>
            </div>

            <div style="margin: 24px auto; width: 140px;">
                <img src="/static/img/bluey.svg" alt="Succès" style="width: 100%; height: auto;">
            </div>

            <p style="color: #555; font-size: 16px; margin-bottom: 32px;">
                Votre compte a été créé avec succès !
            </p>

            <a href="/compte" class="button button-primary full-width"
               style="min-height: 52px; font-size: 1rem; display: inline-flex; align-items: center; justify-content: center; text-decoration: none;">
                Accéder à mon profil
            </a>
        </div>
        "##,
        h(pseudo)
    )
}
