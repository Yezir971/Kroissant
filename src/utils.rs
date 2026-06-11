//! Fonctions utilitaires diverses.

pub fn clean_next(next: Option<String>) -> String {
    next.filter(|value| value.starts_with('/') && !value.starts_with("//") && !value.contains('\n'))
        .unwrap_or_else(|| "/bibliotheque".to_string())
}

pub fn normalize_platform(platform: Option<&str>) -> &str {
    match platform {
        Some("netflix") => "netflix",
        Some("disney") => "disney",
        _ => "youtube",
    }
}

pub fn normalize_tag(tag: Option<&str>) -> Option<String> {
    tag.map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_lowercase())
}
