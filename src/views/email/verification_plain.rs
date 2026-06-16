//! Template texte brut pour l'email de vérification.

pub fn render(token_url: &str) -> String {
    format!(
        r#"Bonjour !

Merci de vous être inscrit sur Kroissant.

Cliquez sur le lien suivant pour vérifier votre adresse e-mail :
{}

Ce lien expire dans 24 heures.

Si vous n'êtes pas à l'origine de cette demande, ignorez cet e-mail.

© 2025 Kroissant"#,
        token_url
    )
}
