//! Utilitaires pour le rendu HTML (escaping).
use html_escape::{encode_double_quoted_attribute, encode_text};

/// Échappe le texte pour une insertion sécurisée dans le contenu HTML.
pub fn h(value: &str) -> String {
    encode_text(value).into_owned()
}

/// Échappe le texte pour une insertion sécurisée dans un attribut HTML (double-quote).
pub fn a(value: &str) -> String {
    encode_double_quoted_attribute(value).into_owned()
}
