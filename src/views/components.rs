//! Composants HTML réutilisables.
use crate::models::{Benefit, Content, TaggedSeries, User};
use crate::views::utils::{a, h};

pub fn render_platform_tabs(active: &str, context: &str) -> String {
    let target = if context == "library" {
        "#library-section"
    } else if context == "home" {
        "#home-platform"
    } else {
        "#platform-results"
    };
    let route = if context == "library" {
        "/partials/library"
    } else {
        "/partials/home"
    };
    let swap = if context == "home" {
        "outerHTML"
    } else {
        "innerHTML"
    };

    let mut tabs = if context == "home" {
        String::from(r#"<div class="platform-tabs home-tabs">"#)
    } else {
        String::from(r#"<div class="platform-tabs">"#)
    };
    for (key, label) in [
        ("youtube", "YouTube"),
        ("netflix", "Netflix"),
        ("disney", "Disney+"),
    ] {
        tabs.push_str(&format!(
            r#"
            <button
                class="platform-tab {}"
                hx-get="{}?platform={}"
                hx-target="{}"
                hx-swap="{}"
                {}
                type="button">{}</button>
            "#,
            if active == key { "active" } else { "" },
            route,
            key,
            target,
            swap,
            if context == "library" {
                format!(r#"hx-push-url="/bibliotheque?platform={}""#, key)
            } else {
                String::new()
            },
            label
        ));
    }
    if context == "home" {
        tabs.push_str(r#"<a class="text-link home-see-all" href="/bibliotheque">Voir tout</a>"#);
    }
    tabs.push_str("</div>");
    tabs
}

pub fn render_home_platform_section(active: &str, contents: &[Content]) -> String {
    format!(
        r#"
        <div id="home-platform">
            {}
            <div id="platform-results" class="card-grid two-cols">
                {}
            </div>
        </div>
        "#,
        render_platform_tabs(active, "home"),
        render_cards(contents)
    )
}

pub fn render_library_section(
    active_tag: Option<&str>,
    tags: &[String],
    series: &[TaggedSeries],
) -> String {
    format!(
        r#"
        <div class="section-heading library-title">
            <h1>Series categorisees par IA</h1>
            <p>Recherche par tags ou par titres, avec des categories calculees au niveau serie.</p>
        </div>
        {}
        <div class="card-grid library-grid ai-series-grid">
            {}
        </div>
        "#,
        render_tag_search(active_tag, tags),
        render_tagged_series_or_empty(series, active_tag),
    )
}

pub fn render_tag_search(active_tag: Option<&str>, tags: &[String]) -> String {
    let value = active_tag.unwrap_or("");
    let mut chips = String::new();

    for tag in tags {
        let href = format!("/bibliotheque?tag={}", a(tag));
        chips.push_str(&format!(
            r#"<a class="tag-chip {}" href="{}">{}</a>"#,
            if active_tag == Some(tag.as_str()) {
                "active"
            } else {
                ""
            },
            href,
            h(tag)
        ));
    }

    format!(
        r#"
        <form class="tag-search-form" method="get" action="/bibliotheque">
            <label for="tag-search">Chercher par titre ou tag</label>
            <div>
                <input id="tag-search" name="tag" value="{}" placeholder="Bluey, empathie, resilience...">
                <button class="button button-secondary" type="submit">Chercher</button>
                <a class="button button-light" href="/bibliotheque">Effacer</a>
            </div>
        </form>
        <div class="tag-chip-row">
            {}
        </div>
        "#,
        a(value),
        chips
    )
}

pub fn render_cards(contents: &[Content]) -> String {
    contents
        .iter()
        .map(render_content_card)
        .collect::<Vec<_>>()
        .join("")
}

pub fn render_cards_or_empty(contents: &[Content], empty: &str) -> String {
    if contents.is_empty() {
        format!(r#"<p class="empty-state">{}</p>"#, h(empty))
    } else {
        render_cards(contents)
    }
}

pub fn render_tagged_series_or_empty(series: &[TaggedSeries], active_tag: Option<&str>) -> String {
    if series.is_empty() {
        let message = match active_tag {
            Some(tag) => format!("Aucune serie trouvee pour \"{}\".", h(tag)),
            None => "Aucune serie categorisee pour l'instant. Lancez le script TMDb/Ollama pour alimenter cette section.".to_string(),
        };
        format!(r#"<p class="empty-state">{}</p>"#, message)
    } else {
        series
            .iter()
            .map(render_tagged_series_card)
            .collect::<Vec<_>>()
            .join("")
    }
}

pub fn render_tagged_series_card(series: &TaggedSeries) -> String {
    let poster = series
        .poster_path
        .as_ref()
        .map(|path| tmdb_image_url(path))
        .unwrap_or_else(|| "/static/img/storybots.svg".to_string());
    let tags = series
        .tags
        .as_deref()
        .unwrap_or("")
        .split(',')
        .filter(|tag| !tag.trim().is_empty())
        .map(|tag| format!(r#"<span class="tag-pill">{}</span>"#, h(tag.trim())))
        .collect::<Vec<_>>()
        .join("");
    let platform = if series.platform.trim().is_empty() {
        "Plateforme a definir".to_string()
    } else {
        platform_label(&series.platform).to_string()
    };
    let age = if series.age_range.trim().is_empty() {
        "Age a definir"
    } else {
        &series.age_range
    };
    let confidence = series
        .confidence
        .map(|value| format!(" · confiance {:.0}%", value * 100.0))
        .unwrap_or_default();
    let first_air_date = series.first_air_date.as_deref().unwrap_or("date inconnue");

    format!(
        r#"
        <article class="content-card ai-series-card" data-series-id="{}">
            <a class="thumb-link" href="{}" target="_blank" rel="noreferrer">
                <img src="{}" alt="">
            </a>
            <div class="card-body">
                <h3><a href="{}" target="_blank" rel="noreferrer">{}</a></h3>
                <p>{} · {} · {}</p>
                <p>{} episode(s) utilises comme contexte{}</p>
                <p>{}</p>
                <div class="tag-list">{}</div>
                <small>TMDb {} · {}</small>
            </div>
        </article>
        "#,
        series.id,
        a(&series.source_url),
        a(&poster),
        a(&series.source_url),
        h(&series.name),
        h(&platform),
        h(age),
        h(first_air_date),
        series.episode_context_count,
        h(&confidence),
        h(&series.overview),
        tags,
        series.tmdb_id,
        h(&series.llm_reason),
    )
}

fn tmdb_image_url(path: &str) -> String {
    if path.starts_with("http://") || path.starts_with("https://") || path.starts_with("/static/") {
        path.to_string()
    } else if path.starts_with('/') {
        format!("https://image.tmdb.org/t/p/w500{path}")
    } else {
        format!("https://image.tmdb.org/t/p/w500/{path}")
    }
}

pub fn render_content_card(content: &Content) -> String {
    let benefit = Benefit::for_skill(&content.skill);
    format!(
        r#"
        <article class="content-card">
            <a class="thumb-link" href="/contenu/{}">
                <img src="{}" alt="">
                <span class="pill floating-pill {}">{}</span>
            </a>
            <div class="card-body">
                <h3><a href="/contenu/{}">{}</a></h3>
                <p>{} · {}</p>
                <a class="button button-primary card-watch" href="/go/{}">Regarder sur {}</a>
            </div>
        </article>
        "#,
        a(&content.slug),
        a(&content.image_url),
        h(benefit.key),
        h(benefit.label),
        a(&content.slug),
        h(&content.title),
        h(&content.duration),
        h(&content.age_range),
        a(&content.slug),
        h(content.platform_label())
    )
}

pub fn render_save_panel(content: &Content, user: Option<&User>, saved: bool) -> String {
    match user {
        Some(_) => {
            let favorite_json = serde_json::to_string(content).unwrap_or_else(|_| "{}".to_string());
            format!(
                r##"
                <div id="save-panel" class="save-panel">
                    <button class="button button-secondary favorite-button" type="button" data-favorite="{}" onclick="saveKroissantFavorite(this)">Mettre en favoris</button>
                    <p class="favorite-feedback" aria-live="polite">{}</p>
                    <script>
                        function saveKroissantFavorite(button) {{
                            const item = JSON.parse(button.dataset.favorite);
                            const key = "kroissant:favorites";
                            const current = JSON.parse(localStorage.getItem(key) || "[]");
                            const next = current.filter((favorite) => favorite.slug !== item.slug);
                            next.unshift({{ ...item, saved_at: new Date().toISOString() }});
                            localStorage.setItem(key, JSON.stringify(next));
                            button.textContent = "Ajoute aux favoris";
                            const panel = button.closest("#save-panel");
                            const feedback = panel && panel.querySelector(".favorite-feedback");
                            if (feedback) feedback.textContent = "Ce contenu est sauvegarde dans ce navigateur.";
                        }}
                    </script>
                </div>
                "##,
                a(&favorite_json),
                if saved {
                    "Ce contenu existe deja dans votre compte parent."
                } else {
                    "Sauvegarde locale dans le navigateur."
                }
            )
        }
        None => format!(
            r#"
            <div id="save-panel" class="save-panel">
                <a class="button button-secondary favorite-button" href="/inscription?next=/contenu/{}">Mettre en favoris</a>
                <p><a href="/inscription?next=/contenu/{}">Creez votre compte</a> pour retrouver vos favoris a chaque visite.</p>
            </div>
            "#,
            a(&content.slug),
            a(&content.slug)
        ),
    }
}

fn platform_label(platform: &str) -> &'static str {
    match platform {
        "youtube" => "YouTube",
        "netflix" => "Netflix",
        "disney" => "Disney+",
        _ => "Tous",
    }
}
