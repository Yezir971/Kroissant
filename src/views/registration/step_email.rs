//! Vue pour l'étape de saisie de l'email.
use crate::views::utils::h;

pub fn render(error: Option<&str>) -> String {
    let error_html = match error {
        Some(msg) => format!(
            r#"<div style="background-color: #FEE2E2; color: #B91C1C; border-radius: 6px; padding: 10px; margin-bottom: 16px; font-size: 0.9rem;">{}</div>"#,
            h(msg)
        ),
        None => "".to_string(),
    };

    format!(
        r##"
        <div style="padding: 40px; position: relative;">
            <button hx-get="/inscription/step/entry"
                    hx-target="#registration-container"
                    hx-swap="innerHTML"
                    data-direction="back"
                    style="background: none; border: none; color: #888; font-size: 14px; cursor: pointer; padding: 0; margin-bottom: 24px; display: flex; align-items: center; gap: 4px;">
                ← Retour
            </button>

            <div style="text-align: center; margin-bottom: 32px;">
                <h1 style="font-family: var(--font-title); font-size: 1.8rem; margin-bottom: 8px;">Vous êtes nouveau ?</h1>
                <p style="color: var(--muted); font-size: 1.1rem;">Créez votre compte</p>
            </div>

            <form hx-post="/inscription/step/email"
                  hx-target="#registration-container"
                  hx-swap="innerHTML"
                  style="display: grid; gap: 16px; margin-bottom: 32px;">
                
                <div style="display: grid; gap: 8px;">
                    <label style="color: var(--brand-dark); font-weight: 800; font-size: 0.86rem;">E-mail</label>
                    <input type="email" name="email" placeholder="JeanDupont@gmail.com"
                           required autofocus
                           style="width: 100%; min-height: 50px; border: 1px solid #d6c9bd; border-radius: 8px; padding: 0 14px; background: #faf5ee; color: var(--ink);">
                </div>

                {}

                <button type="submit" class="button button-primary full-width"
                        style="min-height: 52px; font-size: 1rem;">
                    Vérifier l'e-mail
                </button>
            </form>

            <div style="text-align: center; font-size: 12px; color: #888; line-height: 1.4;">
                En utilisant Kroissant, vous acceptez les <a href="#" style="color: var(--brand); text-decoration: underline;">Conditions d'utilisation</a>
                et la <a href="#" style="color: var(--brand); text-decoration: underline;">Politique de confidentialité</a>.
            </div>
        </div>
        "##,
        error_html
    )
}
