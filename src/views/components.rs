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

pub fn render_home_platform_section(_active: &str, contents: &[Content]) -> String {
    format!(
        r#"
        <div id="home-platform">
            <div class="platform-tabs home-tabs">
                <a class="text-link home-see-all" href="/bibliotheque">Voir tout</a>
            </div>
            <div id="platform-results" class="card-grid two-cols">
                {}
            </div>
        </div>
        "#,
        render_cards(contents)
    )
}

pub fn render_library_section(
    query: &crate::models::PlatformQuery,
    tags: &[String],
    series: &[TaggedSeries],
) -> String {
    format!(
        r##"
        <div class="section-heading library-title">
            <h1>Séries catégorisées par IA</h1>
            <p>Recherche par tags ou par titres, avec des catégories calculées au niveau série.</p>
        </div>
        {}
        <div class="card-grid library-grid ai-series-grid">
            {}
        </div>
        "##,
        render_search_and_filters(query, tags),
        render_tagged_series_or_empty(series, query.tag.as_deref().or(query.skill.as_deref())),
    )
}

pub fn render_search_and_filters(query: &crate::models::PlatformQuery, tags: &[String]) -> String {
    let tag_value = query.tag.as_deref().unwrap_or("");
    let active_age = query.age.as_deref().unwrap_or("all");
    let active_platform = query.platform.as_deref().unwrap_or("all");
    let active_skill = query.skill.as_deref().unwrap_or("all");

    let mut age_filters = String::new();
    for (val, label) in [("all", "Tout"), ("3-7 ans", "3-7 ans"), ("7-10 ans", "7-10 ans")] {
        age_filters.push_str(&format!(
            r#"<button type="button" class="filter-chip {}" onclick="setLibraryFilter('age', '{}')">{}</button>"#,
            if active_age == val { "active" } else { "" },
            val,
            label
        ));
    }

    let mut skill_filters = String::new();
    skill_filters.push_str(&format!(
        r#"<button type="button" class="filter-chip {}" onclick="setLibraryFilter('skill', 'all')">Tout</button>"#,
        if active_skill == "all" { "active" } else { "" }
    ));
    for tag in tags {
        skill_filters.push_str(&format!(
            r#"<button type="button" class="filter-chip {}" onclick="setLibraryFilter('skill', '{}')">{}</button>"#,
            if active_skill == tag { "active" } else { "" },
            tag,
            h(tag)
        ));
    }

    let mut platform_filters = String::new();
    for (val, label) in [("all", "Tout"), ("youtube", "Youtube"), ("netflix", "Netflix"), ("disney", "Disney +")] {
        platform_filters.push_str(&format!(
            r#"<button type="button" class="filter-chip {}" onclick="setLibraryFilter('platform', '{}')">{}</button>"#,
            if active_platform == val { "active" } else { "" },
            val,
            label
        ));
    }

    format!(
        r##"
        <div class="library-controls">
            <form id="library-filter-form" class="search-filter-row" method="get" action="/bibliotheque" hx-get="/partials/library" hx-target="#library-section" hx-push-url="true" hx-trigger="submit, filterChanged">
                <input type="hidden" name="age" id="filter-age" value="{}">
                <input type="hidden" name="platform" id="filter-platform" value="{}">
                <input type="hidden" name="skill" id="filter-skill" value="{}">
                
                <div class="search-input-wrapper">
                    <span class="search-icon-left">
                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg>
                    </span>
                    <input type="text" name="tag" value="{}" placeholder="Recherche" autocomplete="off" hx-trigger="keyup changed delay:500ms" hx-get="/partials/library" hx-target="#library-section">
                    <span class="mic-icon-right">
                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z"></path><path d="M19 10v2a7 7 0 0 1-14 0v-2"></path><line x1="12" y1="19" x2="12" y2="23"></line><line x1="8" y1="23" x2="16" y2="23"></line></svg>
                    </span>
                </div>
                
                <button type="button" class="filter-button" onclick="document.getElementById('filter-panel').classList.toggle('hidden')">
                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="4" y1="21" x2="4" y2="14"></line><line x1="4" y1="10" x2="4" y2="3"></line><line x1="12" y1="21" x2="12" y2="12"></line><line x1="12" y1="8" x2="12" y2="3"></line><line x1="20" y1="21" x2="20" y2="16"></line><line x1="20" y1="12" x2="20" y2="3"></line><line x1="1" y1="14" x2="7" y2="14"></line><line x1="9" y1="8" x2="15" y2="8"></line><line x1="17" y1="16" x2="23" y2="16"></line></svg>
                    Filtres
                </button>
            </form>
            
            <div id="filter-panel" class="filter-panel hidden">
                <div class="filter-group">
                    {}
                </div>
                <div class="filter-group">
                    {}
                </div>
                <div class="filter-group">
                    {}
                </div>
            </div>
            
            <script>
                function setLibraryFilter(name, value) {{
                    document.getElementById('filter-' + name).value = value;
                    const form = document.getElementById('library-filter-form');
                    htmx.trigger(form, 'filterChanged');
                }}
            </script>
        </div>
        "##,
        a(active_age),
        a(active_platform),
        a(active_skill),
        a(tag_value),
        age_filters,
        skill_filters,
        platform_filters
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
