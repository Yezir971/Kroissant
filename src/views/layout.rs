//! Mise en page globale (Layout).
use crate::models::User;
use crate::views::utils::{a, h};

pub fn render_page(
    title: &str,
    left_label: &str,
    back_href: Option<&str>,
    user: &Option<User>,
    body: String,
) -> String {
    render_page_with_class(title, left_label, back_href, user, body, "")
}

pub fn render_page_with_class(
    title: &str,
    left_label: &str,
    back_href: Option<&str>,
    user: &Option<User>,
    body: String,
    page_class: &str,
) -> String {
    let left = match back_href {
        Some(href) => format!(
            r#"<a class="back-link" href="{}">← {}</a>"#,
            a(href),
            h(left_label)
        ),
        None => format!(r#"<span class="muted-nav">{}</span>"#, h(left_label)),
    };

    let nav = match user {
        Some(user) => format!(
            r#"
            <a class="nav-link" href="/compte">{}</a>
            <form method="post" action="/deconnexion"><button class="nav-link ghost-button" type="submit">Deconnexion</button></form>
            "#,
            h(&user.email)
        ),
        None => r#"
            <a class="nav-link" href="/connexion">Connexion</a>
            <a class="nav-button" href="/inscription">S'inscrire</a>
        "#
        .to_string(),
    };

    format!(
        r##"<!doctype html>
        <html lang="fr">
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <title>{}</title>
            <link rel="stylesheet" href="/static/app.css">
            <link rel="manifest" href="/manifest.json">
            <meta name="theme-color" content="#d06b12">
            <meta name="apple-mobile-web-app-capable" content="yes">
            <meta name="apple-mobile-web-app-status-bar-style" content="black-translucent">
            <meta name="apple-mobile-web-app-title" content="Ploopy">
            <link rel="apple-touch-icon" href="/static/img/bluey.svg">
            <script src="/static/htmx.min.js" defer></script>
            <script src="/static/pwa-install.js" defer></script>
            <script>
                if ('serviceWorker' in navigator) {{
                    window.addEventListener('load', () => {{
                        navigator.serviceWorker.register('/sw.js');
                    }});
                }}
            </script>
        </head>
        <body class="{}">
            <header class="topbar">
                <div class="topbar-left">{}</div>
                <a class="brand" href="/" aria-label="Accueil Ploopy">
                    <img src="/static/img/ploopy.svg" alt="Ploopy">
                </a>
                <nav class="topbar-nav">{}</nav>
            </header>
            {}
            <footer class="site-footer">
                <a class="footer-brand" href="/">
                                    <img src="/static/img/ploopy.svg" alt="Ploopy">
                                    </a>
                <p>© 2026 Ploopy - Tous droits reserves.</p>
                <a class="footer-link" href="/science">Comment sont choisis nos contenus</a>
            </footer>
        </body>
        </html>"##,
        h(title),
        a(page_class),
        left,
        nav,
        body
    )
}
