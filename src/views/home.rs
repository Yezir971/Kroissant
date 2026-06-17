//! Page d'accueil.
use crate::models::{Content, User};
use crate::views::components::{render_cards, render_home_platform_section};
use crate::views::layout::render_page_with_class;

pub fn render_home(user: &Option<User>, selected: &[Content], moment: &[Content]) -> String {
    let body = format!(
        r#"
        <section class="hero">
            <div class="hero-inner">
                <div class="hero-copy">
                    <h1>La plateforme qui selectionne les
dessins animes adaptes a votre enfant.</h1>
                    <p class="hero-copy-text">Chaque contenu est evalue selon des criteres pedagogiques avant d'apparaitre dans la bibliotheque. Vous parcourez, vous choisissez, vous regardez. Tout ce qui s'affiche est deja verifie.</p>
                    <div class="hero-actions">
                        <a class="button button-primary" href="/inscription">Je m'inscris et decouvre</a>
                        <a class="button button-outline-brown" href="/science">Comment sont choisis nos contenus</a>
                        <button id="install-button" class="button button-primary" style="display: none;">Installer l'application</button>
                    </div>
                </div>
                <img class="hero-illustration" src="/static/img/illustration.svg" alt="" aria-hidden="true">
            </div>
        </section>

        <main class="page-flow">
            <section class="section-block">
                <div class="section-heading split-heading">
                    <div>
                        <h2>Choisissez votre contenu</h2>
                    </div>
                    <div>
                        <a class="text-link home-see-all" href="/bibliotheque">Voir tout</a>
                    </div>
                </div>
                {}
            </section>

            <section class="how-it-works">
                <div class="how-it-works-grid">
                    <div class="how-info">
                        <h2>Comment ca marche</h2>
                        <p>Trois etapes, zero mauvaise surprise.</p>
                    </div>
                    <div class="how-steps">
                        <div class="step-item">
                            <span class="step-num">1</span>
                            <div>
                                <h3>Parcourez la bibliotheque</h3>
                                <p>Des dessins animes ranges par valeur : courage, empathie, langage...</p>
                            </div>
                        </div>
                        <div class="step-item">
                            <span class="step-num">2</span>
                            <div>
                                <h3>Tout est deja verifie, vous pouvez en profiter</h3>
                                <p>Chaque contenu passe nos criteres pediatriques avant d'apparaitre.</p>
                            </div>
                        </div>
                        <div class="step-item">
                            <span class="step-num">3</span>
                            <div>
                                <h3>Regardez l'esprit tranquille</h3>
                                <p>Vous lancez le contenu sur la plateforme de votre choix, sans surprise.</p>
                            </div>
                        </div>
                    </div>
                </div>
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

        "#,
        render_home_platform_section("youtube", selected),
        render_cards(moment),
    );

    render_page_with_class("Ploopy", "Accueil", None, user, body, "home-page")
}
