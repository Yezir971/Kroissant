//! Page d'accueil.
use crate::models::{Content, User};
use crate::views::components::{render_home_platform_section, render_cards};
use crate::views::layout::render_page;

pub fn render_home(user: &Option<User>, selected: &[Content], moment: &[Content]) -> String {
    let body = format!(
        r#"
        <section class="hero">
            <div class="hero-inner">
                <h1>Des dessins animés choisis pour eux. Une sérénité retrouvée pour vous.</h1>
                <p>Une bibliotheque de contenus selectionnes pour leurs benefices developpementaux prouves: langage, empathie, resilience, creativite.</p>
                <div class="hero-actions">
                    <a class="button button-light" href="/bibliotheque">Decouvrir les contenus</a>
                    <a class="button button-outline-light" href="/science">Comment sont choisis nos contenus</a>
                    <button id="install-button" class="button button-primary" style="display: none;">Installer l'application</button>
                </div>
            </div>
        </section>

        <main class="page-flow">
            <section class="section-block">
                <div class="section-heading split-heading">
                    <div>
                        <h2>Choisissez votre plateforme</h2>
                    </div>
                </div>
                {}
            </section>

            <section class="section-block">
                <div class="section-heading">
                    <h2>Selection du moment</h2>
                    <p>Tous selectionnes pour leur valeur developpementale</p>
                </div>
                <div class="card-grid two-cols">
                    {}
                </div>
            </section>
        </main>

        <section class="stat-band">
            <span>98% des enfants 4-14 ans regardent des dessins animes</span>
            <span>Certains developpent autant que des activites structurees</span>
        </section>

        <section class="how-it-works">
            <h2>Comment ca fonctionne</h2>
            <div class="steps">
                <div><strong>1</strong><h3>Choisissez votre plateforme</h3><p>YouTube, Netflix ou Disney+</p></div>
                <div><strong>2</strong><h3>Parcourez la bibliotheque</h3><p>Chaque contenu explique ce qu'il developpe</p></div>
                <div><strong>3</strong><h3>Redirige en 1 clic</h3><p>Directement sur la plateforme</p></div>
            </div>
        </section>
        "#,
        render_home_platform_section("youtube", selected),
        render_cards(moment),
    );

    render_page("Kroissant", "Accueil", None, user, body)
}
