//! Page de la bibliothèque.
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
                        <img class="library-carousel-slide" src="/static/img/puffin-rock.svg" alt="image du dessin animé Puffin Rock">
                        <img class="library-carousel-slide" src="/static/img/bluey.svg" alt="image du dessin animé Bluey">
                        <img class="library-carousel-slide" src="/static/img/tumble-leaf.svg" alt="image du dessin animé Tumble Leaf">
                        <img class="library-carousel-slide" src="/static/img/ada-twist.svg" alt="image du dessin animé Ada Twist">
                        <img class="library-carousel-slide" src="/static/img/baymax.svg" alt="image du dessin animé Baymax">
                        <img class="library-carousel-slide" src="/static/img/puffin-rock.svg" alt="image du dessin animé Puffin Rock">
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
