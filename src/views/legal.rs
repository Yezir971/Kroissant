//! Pages legales publiques.
use crate::models::User;
use crate::views::layout::render_page_with_class;

const LEGAL_CONTACT: &str = "contact@kroissant.fr";
const UPDATED_AT: &str = "18 juin 2026";

pub fn render_privacy(user: &Option<User>) -> String {
    let body = format!(
        r#"
        <main class="legal-shell">
            <section class="legal-hero">
                <p class="legal-kicker">Derniere mise a jour : {1}</p>
                <h1>Politique de confidentialite</h1>
                <p>Ploopy aide les parents a trouver des contenus adaptes aux enfants. Cette politique explique quelles donnees nous traitons, pourquoi, combien de temps nous les conservons et comment exercer vos droits.</p>
            </section>

            <section class="legal-content">
                <article>
                    <h2>1. Responsable du traitement</h2>
                    <p>Le service Ploopy est edite par Kroissant. Pour toute question relative a vos donnees personnelles, vous pouvez nous ecrire a <a href="mailto:{0}">{0}</a>.</p>
                </article>

                <article>
                    <h2>2. Donnees collectees</h2>
                    <p>Nous collectons uniquement les informations utiles au fonctionnement du service : adresse e-mail, pseudo, mot de passe chiffre, identifiant de compte, contenus sauvegardes, historique de consultation interne, jetons de verification d'e-mail, donnees techniques de securite et journaux applicatifs.</p>
                    <p>Si vous utilisez la connexion Google, Google nous transmet les informations necessaires a l'authentification, notamment votre adresse e-mail et votre nom de profil.</p>
                </article>

                <article>
                    <h2>3. Finalites et bases legales</h2>
                    <ul>
                        <li>Creation et gestion du compte : execution du service demande.</li>
                        <li>Connexion, verification d'e-mail et securite : execution du service et interet legitime de protection de la plateforme.</li>
                        <li>Sauvegarde de contenus et personnalisation de la bibliotheque : execution du service.</li>
                        <li>Mesure technique, correction d'erreurs et prevention des abus : interet legitime.</li>
                        <li>Respect d'obligations legales ou reponse aux autorites habilitees : obligation legale.</li>
                    </ul>
                </article>

                <article>
                    <h2>4. Cookies et stockage local</h2>
                    <p>Ploopy utilise des cookies strictement necessaires, notamment pour maintenir votre session et securiser l'authentification. L'application peut aussi utiliser des mecanismes techniques lies a la PWA, comme le service worker, afin d'ameliorer le chargement et l'experience hors ligne.</p>
                    <p>Nous n'utilisons pas de cookies publicitaires dans la version actuelle du service.</p>
                </article>

                <article>
                    <h2>5. Destinataires</h2>
                    <p>Vos donnees sont accessibles uniquement aux personnes et prestataires qui en ont besoin pour exploiter Ploopy : hebergement, base de donnees, envoi d'e-mails transactionnels, authentification et maintenance. Ces prestataires agissent selon nos instructions lorsque cela est requis.</p>
                </article>

                <article>
                    <h2>6. Durees de conservation</h2>
                    <ul>
                        <li>Compte utilisateur : pendant la duree d'utilisation du compte, puis suppression ou anonymisation apres demande de suppression ou inactivite prolongee.</li>
                        <li>Contenus sauvegardes et preferences : pendant la duree de vie du compte.</li>
                        <li>Jetons de verification d'e-mail : duree courte necessaire a la verification, puis invalidation.</li>
                        <li>Journaux techniques : duree limitee necessaire a la securite, au diagnostic et a la prevention des abus.</li>
                        <li>Donnees devant etre archivees pour une obligation legale : pendant la duree imposee par le texte applicable.</li>
                    </ul>
                </article>

                <article>
                    <h2>7. Securite</h2>
                    <p>Nous mettons en place des mesures raisonnables pour proteger les donnees : mots de passe haches, cookies de session HTTPOnly, limitation des acces techniques, separation des secrets de configuration et journalisation sans donnees sensibles lorsque possible.</p>
                </article>

                <article>
                    <h2>8. Vos droits</h2>
                    <p>Vous pouvez demander l'acces, la rectification, l'effacement, la limitation, l'opposition ou la portabilite de vos donnees lorsque ces droits s'appliquent. Ecrivez-nous a <a href="mailto:{0}">{0}</a>. Vous pouvez egalement introduire une reclamation aupres de la CNIL.</p>
                </article>

                <article>
                    <h2>9. Enfants</h2>
                    <p>Ploopy est destine aux parents et responsables legaux. Les comptes doivent etre crees et administres par un adulte. Nous ne cherchons pas a collecter directement des donnees aupres d'enfants.</p>
                </article>

                <article>
                    <h2>10. Evolution de cette politique</h2>
                    <p>Nous pouvons modifier cette politique pour tenir compte de l'evolution du service, de la loi ou de nos prestataires. En cas de changement important, nous ferons nos meilleurs efforts pour vous en informer.</p>
                </article>
            </section>
        </main>
        "#,
        LEGAL_CONTACT, UPDATED_AT,
    );

    render_page_with_class(
        "Politique de confidentialite - Ploopy",
        "Accueil",
        Some("/"),
        user,
        body,
        "legal-page",
    )
}

pub fn render_terms(user: &Option<User>) -> String {
    let body = format!(
        r#"
        <main class="legal-shell">
            <section class="legal-hero">
                <p class="legal-kicker">Derniere mise a jour : {1}</p>
                <h1>Conditions d'utilisation</h1>
                <p>Ces conditions definissent les regles d'utilisation de Ploopy. En creant un compte ou en utilisant le service, vous acceptez ces conditions.</p>
            </section>

            <section class="legal-content">
                <article>
                    <h2>1. Objet du service</h2>
                    <p>Ploopy propose une selection editoriale de contenus pour enfants, classee par themes, plateformes et benefices developpementaux. Le service aide les parents a decouvrir, sauvegarder et consulter des recommandations.</p>
                </article>

                <article>
                    <h2>2. Acces au service</h2>
                    <p>L'acces a certaines fonctionnalites peut necessiter la creation d'un compte. Vous devez fournir des informations exactes, garder vos identifiants confidentiels et nous signaler toute utilisation non autorisee de votre compte.</p>
                </article>

                <article>
                    <h2>3. Public concerne</h2>
                    <p>Le service est destine aux adultes, parents ou responsables legaux. Les recommandations ne remplacent pas votre jugement, votre supervision ni les regles d'age ou de controle parental propres a chaque plateforme de diffusion.</p>
                </article>

                <article>
                    <h2>4. Recommandations et contenus tiers</h2>
                    <p>Ploopy ne diffuse pas directement les contenus recommandes. Les liens peuvent renvoyer vers des plateformes tierces comme YouTube, Netflix, Disney+ ou d'autres services. Leur disponibilite, leurs tarifs, leurs classifications et leurs conditions peuvent changer sans que Ploopy en soit responsable.</p>
                </article>

                <article>
                    <h2>5. Usage autorise</h2>
                    <ul>
                        <li>Vous utilisez Ploopy pour un usage personnel et familial.</li>
                        <li>Vous vous interdisez de perturber le service, tenter d'acceder aux donnees d'autrui ou contourner les mesures de securite.</li>
                        <li>Vous ne devez pas utiliser Ploopy pour collecter massivement des donnees, automatiser des requetes abusives ou reproduire la selection editoriale sans autorisation.</li>
                    </ul>
                </article>

                <article>
                    <h2>6. Compte et suppression</h2>
                    <p>Vous pouvez cesser d'utiliser Ploopy a tout moment. Pour demander la suppression de votre compte ou de vos donnees, contactez-nous a <a href="mailto:{0}">{0}</a>.</p>
                </article>

                <article>
                    <h2>7. Propriete intellectuelle</h2>
                    <p>La marque Ploopy, l'interface, les textes, les selections, les classifications et les elements graphiques du service appartiennent a Kroissant ou a leurs titulaires respectifs. Les marques et contenus des plateformes tierces restent la propriete de leurs ayants droit.</p>
                </article>

                <article>
                    <h2>8. Disponibilite</h2>
                    <p>Nous faisons nos meilleurs efforts pour maintenir le service accessible, mais nous ne garantissons pas une disponibilite permanente. Des interruptions peuvent intervenir pour maintenance, mise a jour, panne technique ou force majeure.</p>
                </article>

                <article>
                    <h2>9. Responsabilite</h2>
                    <p>Ploopy fournit des recommandations a titre informatif. Vous restez responsable du choix final des contenus regardes par votre enfant, du temps d'ecran, du contexte de visionnage et de l'utilisation des services tiers.</p>
                </article>

                <article>
                    <h2>10. Modification des conditions</h2>
                    <p>Nous pouvons modifier ces conditions pour adapter le service ou respecter une obligation legale. La version applicable est celle disponible sur cette page au moment de votre utilisation.</p>
                </article>

                <article>
                    <h2>11. Contact</h2>
                    <p>Pour toute question sur ces conditions, vous pouvez nous ecrire a <a href="mailto:{0}">{0}</a>.</p>
                </article>
            </section>
        </main>
        "#,
        LEGAL_CONTACT, UPDATED_AT,
    );

    render_page_with_class(
        "Conditions d'utilisation - Ploopy",
        "Accueil",
        Some("/"),
        user,
        body,
        "legal-page",
    )
}
