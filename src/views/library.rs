//! Page de la bibliotheque.
use crate::models::{TaggedSeries, User};
use crate::views::components::{render_library_section, render_search_and_filters};
use crate::views::layout::render_page;

pub fn render_library(
    user: &Option<User>,
    query: &crate::models::PlatformQuery,
    tags: &[String],
    series: &[TaggedSeries],
) -> String {
    let body = format!(
        r#"
        <main class="library-shell">
            <section class="library-visual" aria-label="Bibliotheque Ploopy">
                {}
                <div class="library-carousel" aria-hidden="true">
                    <div class="library-carousel-track">
                        <picture class="library-carousel-slide">
                            <source media="(max-width: 640px)" srcset="/static/img/puffin_rock_mobile.webp">
                            <img src="/static/img/puffin_rock_desktop.webp" alt="">
                        </picture>
                        <picture class="library-carousel-slide">
                            <source media="(max-width: 640px)" srcset="/static/img/bluey_mobile.webp">
                            <img src="/static/img/blue_desktop.webp" alt="">
                        </picture>
                        <picture class="library-carousel-slide">
                            <source media="(max-width: 640px)" srcset="/static/img/hilda_mobile.webp">
                            <img src="/static/img/hilda_desktop.webp" alt="">
                        </picture>
                        <picture class="library-carousel-slide">
                            <source media="(max-width: 640px)" srcset="/static/img/puffin_rock_mobile.webp">
                            <img src="/static/img/puffin_rock_desktop.webp" alt="">
                        </picture>
                    </div>
                </div>
            </section>
            <section id="library-section">
                {}
            </section>
        </main>
        "#,
        render_search_and_filters(query, tags),
        render_library_section(query, tags, series)
    );

    render_page(
        "Bibliotheque - Ploopy",
        "Accueil",
        Some("/"),
        user,
        body,
    )
}
