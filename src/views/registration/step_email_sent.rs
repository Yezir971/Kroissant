//! Vue pour la confirmation d'envoi de l'email de vérification.
use crate::views::utils::h;

pub fn render(email: &str) -> String {
    format!(
        r##"
        <div style="padding: 40px; position: relative; text-align: center;">
            <button hx-get="/inscription/step/email"
                    hx-target="#registration-container"
                    hx-swap="innerHTML"
                    data-direction="back"
                    style="background: none; border: none; color: #888; font-size: 14px; cursor: pointer; padding: 0; position: absolute; top: 40px; left: 40px;">
                ← Retour
            </button>

            <div style="margin-top: 48px; margin-bottom: 32px; display: flex; justify-content: center;">
                <svg width="48" height="48" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z" stroke="#A0522D" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                    <path d="m22 6-10 7L2 6" stroke="#A0522D" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
            </div>

            <h1 style="font-family: var(--font-title); font-size: 1.8rem; margin-bottom: 16px;">Vérifiez votre boîte mail</h1>
            
            <p style="color: #555; line-height: 1.6; margin-bottom: 32px;">
                Un mail de vérification a été envoyé à <strong>{}</strong>.<br>
                Cliquez sur le bouton de vérification dans l'e-mail pour continuer.
            </p>

            <div style="font-size: 0.9rem; color: var(--muted);">
                Vous n'avez rien reçu ? <br>
                Vérifiez vos spams ou <button hx-post="/inscription/step/email" hx-vals='{{"email": "{}"}}' hx-target="#registration-container" style="background: none; border: none; color: var(--brand); text-decoration: underline; cursor: pointer; padding: 0; font-size: inherit;">renvoyer l'e-mail</button>.
            </div>
        </div>
        "##,
        h(email),
        h(email)
    )
}
