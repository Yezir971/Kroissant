//! Page de la bibliothèque.
use crate::models::{TaggedSeries, User};
use crate::views::components::render_library_section;
use crate::views::layout::render_page;

pub fn render_library(
    user: &Option<User>,
    active_tag: Option<&str>,
    tags: &[String],
    series: &[TaggedSeries],
) -> String {
    let body = format!(
        r#"
        <main class="library-shell">
            <section id="library-section">
                {}
            </section>
        </main>
        "#,
        render_library_section(active_tag, tags, series)
    );

    render_page(
        "Bibliotheque - Kroissant",
        "Accueil",
        Some("/"),
        user,
        body,
    )
}
