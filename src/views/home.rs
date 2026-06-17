//! Page d'accueil.
use crate::models::{Content, User};
use crate::views::components::{render_home_platform_section, render_cards};
use crate::views::layout::render_page;

pub fn render_home(user: &Option<User>, selected: &[Content], moment: &[Content]) -> String {
    let body = format!(
        r#"
        <section class="hero">
            <div class="hero-inner">
                <div class="hero-copy">
                    <h1>La plateforme qui sélectionne les 
dessins animés adaptés à votre enfant.</h1>
                    <p class="hero-copy-text">Chaque contenu est évalué selon des critères pédagogiques avant d'apparaître dans la bibliothèque. Vous parcourez, vous choisissez, vous regardez. Tout ce qui s'affiche est déjà vérifié.</p>
                    <div class="hero-actions">
                        <a class="button button-primary" href="/inscription">Je m'inscris et découvre</a>
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
                        <h2>Comment ça marche</h2>
                        <p>Chaque contenu est évalué selon des critères pédagogiques avant d'apparaître dans la bibliothèque.</p>
                        <div class="testimonials">
                            <p>"tout est bien expliqué, plateforme dédiée aux enfants"</p>
                            <p>"c'est bien explicite, pas du tout complexe, facile d'utilisation"</p>
                        </div>
                        <a href="/science" class="button button-primary">Découvrir notre méthode</a>
                    </div>
                    <div class="how-steps">
                        <div class="step-item">
                            <span class="step-num">1</span>
                            <div>
                                <h3>Se connecter</h3>
                                <p>YouTube, Netflix ou Disney+</p>
                            </div>
                        </div>
                        <div class="step-item">
                            <span class="step-num">2</span>
                            <div>
                                <h3>Choisir un dessin animé</h3>
                                <p>YouTube, Netflix ou Disney+</p>
                            </div>
                        </div>
                        <div class="step-item">
                            <span class="step-num">3</span>
                            <div>
                                <h3>Lancer le dessin animé</h3>
                                <p>YouTube, Netflix ou Disney+</p>
                            </div>
                        </div>
                        <div class="step-item">
                            <span class="step-num">4</span>
                            <div>
                                <h3>Profiter de votre moment</h3>
                                <p>YouTube, Netflix ou Disney+</p>
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

    render_page("Ploopy", "Accueil", None, user, body)
}



