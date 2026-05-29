# Kroissant - application Rust + HTMX

Documentation technique de l'application Kroissant, une app web server-side en Rust pour explorer des contenus animes selectionnes, avec auth JWT, interactions HTMX et donnees de developpement dans SQLite.

Le projet a ete initialise sans Git, comme demande. Il n'y a pas de dossier `.git`.

## Stack technique

| Couche | Choix | Role |
| --- | --- | --- |
| Serveur HTTP | `axum` | Routing, extracteurs, reponses HTML et redirections |
| Runtime async | `tokio` | Execution asynchrone du serveur et de SQLx |
| Base de donnees | SQLite via `sqlx` | Stockage local des utilisateurs, contenus, favoris et historique |
| Auth | JWT via `jsonwebtoken` | Session stateless stockee en cookie HttpOnly |
| Hash mot de passe | `argon2` | Hash securise des mots de passe utilisateur |
| UI dynamique | HTMX vendore localement | Filtres et sauvegardes sans SPA |
| Assets statiques | `tower-http::ServeDir` | Sert le CSS, HTMX et les images SVG locales |

## Prerequis

- Rust recent avec Cargo.
- SQLite CLI optionnel pour inspecter la base.

Verification rapide :

```bash
rustc --version
cargo --version
sqlite3 --version
```

## Lancement en developpement

Depuis la racine du projet :

```bash
cargo run
```

Par defaut, le serveur ecoute sur :

```text
http://127.0.0.1:3000
```

Changer le port :

```bash
PORT=3001 cargo run
```

Configurer explicitement la base et le secret JWT :

```bash
DATABASE_URL=sqlite://data/kroissant.sqlite \
JWT_SECRET=change-me-in-real-env \
cargo run
```

Changer l'adresse d'ecoute, utile en conteneur :

```bash
HOST=0.0.0.0 PORT=3000 cargo run
```

## Lancement avec Docker

Build de l'image :

```bash
docker build -t kroissant:dev .
```

Lancement du conteneur :

```bash
docker run --rm \
  -p 3000:3000 \
  -e JWT_SECRET=change-me-for-local-docker \
  -v kroissant-data:/app/data \
  kroissant:dev
```

L'application est ensuite disponible sur :

```text
http://127.0.0.1:3000
```

Si le serveur local `cargo run` occupe deja le port `3000`, utiliser un autre port cote machine :

```bash
docker run --rm \
  -p 3001:3000 \
  -e JWT_SECRET=change-me-for-local-docker \
  -v kroissant-data:/app/data \
  kroissant:dev
```

URL dans ce cas :

```text
http://127.0.0.1:3001
```

## Lancement avec Docker Compose

Demarrer :

```bash
docker compose up --build
```

Demarrer en arriere-plan :

```bash
docker compose up --build -d
```

Voir les logs :

```bash
docker compose logs -f kroissant
```

Arreter :

```bash
docker compose down
```

Supprimer aussi le volume SQLite :

```bash
docker compose down -v
```

Compose expose le service sur `http://127.0.0.1:3000` et persiste la base dans le volume Docker nomme `kroissant-data`.

## Commandes utiles

```bash
cargo fmt
cargo check
cargo run
```

Inspecter les contenus seedes :

```bash
sqlite3 data/kroissant.sqlite "select platform, count(*) from contents group by platform;"
```

Nettoyer les comptes de test sans supprimer les contenus :

```bash
sqlite3 data/kroissant.sqlite "DELETE FROM saved_items; DELETE FROM watch_history; DELETE FROM users;"
```

## Arborescence

```text
.
+-- Cargo.toml
+-- Cargo.lock
+-- Dockerfile
+-- docker-compose.yml
+-- .dockerignore
+-- README.md
+-- data/
|   +-- kroissant.sqlite
+-- src/
|   +-- main.rs
+-- static/
    +-- app.css
    +-- htmx.min.js
    +-- img/
        +-- bluey.svg
        +-- puffin-rock.svg
        +-- ...
```

## Vue d'ensemble runtime

Au demarrage, `main()` effectue les etapes suivantes :

1. Initialise les logs via `tracing-subscriber`.
2. Cree le dossier `data/` si absent.
3. Ouvre une connexion SQLite avec `create_if_missing(true)` et `foreign_keys(true)`.
4. Execute les migrations idempotentes avec `CREATE TABLE IF NOT EXISTS`.
5. Seed les fausses donnees si la table `contents` est vide.
6. Construit l'etat partage `AppState`.
7. Monte les routes Axum et les assets statiques.
8. Demarre le serveur sur `127.0.0.1:$PORT`, avec `3000` par defaut.

L'application est volontairement server-rendered : les pages HTML sont produites par Rust, et HTMX remplace seulement des fragments cibles.

## Variables d'environnement

| Variable | Defaut | Description |
| --- | --- | --- |
| `HOST` | `127.0.0.1` en local, `0.0.0.0` dans Docker | Adresse d'ecoute HTTP |
| `PORT` | `3000` | Port HTTP |
| `DATABASE_URL` | `sqlite://data/kroissant.sqlite` | URL SQLx vers SQLite |
| `JWT_SECRET` | `dev-secret-change-me-kroissant` | Secret HMAC des JWT |
| `RUST_LOG` | `tower_http=debug,kroissant=debug` | Niveau de logs |

Important : le `JWT_SECRET` par defaut est seulement acceptable en developpement local.

## Details Docker

Le `Dockerfile` est multi-stage :

1. Stage `builder` base sur `rust:1.92-bookworm`.
2. Compilation release avec `cargo build --release --locked`.
3. Stage runtime base sur `debian:bookworm-slim`.
4. Installation de `ca-certificates`.
5. Creation d'un utilisateur systeme non-root `app`.
6. Copie du binaire release sous `/usr/local/bin/kroissant`.
7. Copie des assets `static/`.
8. Creation du dossier `/app/data`.
9. Demarrage avec `CMD ["kroissant"]`.

Le conteneur n'embarque pas la base locale `data/kroissant.sqlite` grace a `.dockerignore`. Au premier demarrage, l'application cree `/app/data/kroissant.sqlite`, execute les migrations, puis seed les contenus.

Variables definies dans l'image :

| Variable | Valeur image |
| --- | --- |
| `HOST` | `0.0.0.0` |
| `PORT` | `3000` |
| `DATABASE_URL` | `sqlite://data/kroissant.sqlite` |
| `JWT_SECRET` | `dev-secret-change-me-kroissant` |
| `RUST_LOG` | `info` |

En production, fournir toujours `JWT_SECRET` au lancement et ne pas conserver la valeur par defaut.

## Routes HTTP

| Methode | Route | Description |
| --- | --- | --- |
| `GET` | `/` | Page d'accueil |
| `GET` | `/bibliotheque` | Bibliotheque filtrable par plateforme |
| `GET` | `/partials/home?platform=...` | Fragment HTMX du filtre accueil |
| `GET` | `/partials/library?platform=...` | Fragment HTMX du filtre bibliotheque |
| `GET` | `/contenu/{slug}` | Fiche detaillee d'un contenu |
| `POST` | `/contenu/{id}/save` | Toggle favoris via HTMX |
| `GET` | `/go/{slug}` | Enregistre l'historique si connecte puis redirige vers la plateforme |
| `GET` | `/science` | Page de justification scientifique |
| `GET` | `/inscription` | Formulaire d'inscription |
| `POST` | `/inscription` | Creation compte + cookie JWT |
| `GET` | `/connexion` | Formulaire de connexion |
| `POST` | `/connexion` | Verification mot de passe + cookie JWT |
| `POST` | `/deconnexion` | Suppression du cookie JWT |
| `GET` | `/compte` | Compte parent, favoris et historique |
| `GET` | `/static/*` | Assets statiques |

## Parametres de filtre

Les plateformes supportees sont :

- `youtube`
- `netflix`
- `disney`
- `all` pour la bibliotheque uniquement

Exemples :

```text
/bibliotheque?platform=all
/bibliotheque?platform=netflix
/partials/home?platform=disney
```

Les valeurs inconnues sont normalisees :

- accueil : fallback sur `youtube`
- bibliotheque : fallback sur `all`

## Schema SQLite

Les migrations sont codees dans `migrate()` dans `src/main.rs`.

### `users`

| Colonne | Type | Contrainte |
| --- | --- | --- |
| `id` | `INTEGER` | Primary key autoincrement |
| `email` | `TEXT` | Unique, not null |
| `password_hash` | `TEXT` | Not null |
| `created_at` | `TEXT` | ISO/RFC3339, not null |

### `contents`

| Colonne | Type | Contrainte |
| --- | --- | --- |
| `id` | `INTEGER` | Primary key autoincrement |
| `slug` | `TEXT` | Unique, not null |
| `title` | `TEXT` | Not null |
| `platform` | `TEXT` | Not null |
| `duration` | `TEXT` | Not null |
| `age_range` | `TEXT` | Not null |
| `description` | `TEXT` | Not null |
| `skill` | `TEXT` | Not null |
| `image_url` | `TEXT` | Not null |
| `source_url` | `TEXT` | Not null |
| `sort_order` | `INTEGER` | Not null |

### `saved_items`

| Colonne | Type | Contrainte |
| --- | --- | --- |
| `user_id` | `INTEGER` | Foreign key vers `users(id)` |
| `content_id` | `INTEGER` | Foreign key vers `contents(id)` |
| `created_at` | `TEXT` | ISO/RFC3339, not null |

Cle primaire composee :

```sql
PRIMARY KEY (user_id, content_id)
```

Les suppressions utilisateur ou contenu suppriment les favoris associes via `ON DELETE CASCADE`.

### `watch_history`

| Colonne | Type | Contrainte |
| --- | --- | --- |
| `id` | `INTEGER` | Primary key autoincrement |
| `user_id` | `INTEGER` | Foreign key vers `users(id)` |
| `content_id` | `INTEGER` | Foreign key vers `contents(id)` |
| `watched_at` | `TEXT` | ISO/RFC3339, not null |

## Seed de donnees

Le seed est gere par `seed_fake_data()`.

Regle actuelle :

- si `contents` contient deja au moins une ligne, aucun seed n'est rejoue ;
- sinon, 12 contenus fictifs sont inseres.

Repartition actuelle :

| Plateforme | Nombre |
| --- | ---: |
| `youtube` | 5 |
| `netflix` | 5 |
| `disney` | 2 |

Pour regenerer proprement les contenus, supprimer la base puis relancer :

```bash
rm data/kroissant.sqlite
cargo run
```

## Authentification JWT

### Creation de compte

Flux `POST /inscription` :

1. Normalise l'email en minuscules.
2. Refuse un mot de passe de moins de 8 caracteres.
3. Verifie l'unicite email.
4. Hash le mot de passe avec Argon2.
5. Cree l'utilisateur.
6. Genere un JWT valable 7 jours.
7. Pose le cookie `kroissant_token`.
8. Redirige vers `next` ou `/bibliotheque`.

### Connexion

Flux `POST /connexion` :

1. Charge l'utilisateur par email.
2. Verifie le mot de passe avec Argon2.
3. Genere un JWT.
4. Pose le cookie.
5. Redirige vers `next` ou `/bibliotheque`.

### Cookie

Nom :

```text
kroissant_token
```

Attributs :

```text
HttpOnly; SameSite=Lax; Path=/; Max-Age=604800
```

Le cookie n'a pas l'attribut `Secure` car l'application tourne en HTTP local. En production HTTPS, ajouter `Secure`.

### Claims JWT

```json
{
  "sub": 1,
  "email": "parent@example.com",
  "exp": 1780648566
}
```

`sub` contient l'identifiant utilisateur.

### Verification session

`current_user()` :

1. Lit le cookie `kroissant_token`.
2. Decode et valide le JWT avec `JWT_SECRET`.
3. Charge l'utilisateur en base via `claims.sub`.
4. Retourne `None` si le cookie est absent, invalide ou si l'utilisateur n'existe plus.

### Securite actuelle

Deja en place :

- mots de passe hashes avec Argon2 ;
- JWT signe cote serveur ;
- cookie HttpOnly ;
- `SameSite=Lax` ;
- validation basique de `next` pour eviter les redirections externes.

A ajouter avant production :

- `Secure` sur le cookie en HTTPS ;
- secret JWT fort fourni par l'environnement ;
- protection CSRF explicite pour les `POST` sensibles ;
- rate limiting sur inscription et connexion ;
- validation email plus stricte ;
- migrations versionnees au lieu de migrations inline ;
- politique de logs sans fuite de donnees sensibles.

## HTMX

HTMX est servi localement depuis :

```text
static/htmx.min.js
```

Il est charge sur toutes les pages par `render_page()`.

### Filtre accueil

Les boutons plateforme appellent :

```text
GET /partials/home?platform=...
```

Le fragment remplace :

```text
#home-platform
```

avec :

```text
hx-swap="outerHTML"
```

Cela permet de mettre a jour a la fois les cartes et l'etat actif du bouton.

### Filtre bibliotheque

Les boutons appellent :

```text
GET /partials/library?platform=...
```

Le fragment remplace :

```text
#library-section
```

avec :

```text
hx-swap="innerHTML"
```

Les boutons poussent aussi l'URL :

```text
hx-push-url="/bibliotheque?platform=..."
```

### Sauvegarde contenu

Sur une fiche contenu, le bouton de sauvegarde appelle :

```text
POST /contenu/{id}/save
```

Le fragment remplace :

```text
#save-panel
```

avec :

```text
hx-swap="outerHTML"
```

Comportement :

- utilisateur non connecte : renvoie un panneau avec lien d'inscription ;
- utilisateur connecte : toggle `saved_items`, puis renvoie le nouveau bouton.

## Rendu HTML

Les vues sont generees par fonctions Rust dans `src/main.rs`.

Fonctions principales :

| Fonction | Role |
| --- | --- |
| `render_page()` | Layout HTML global, topbar, CSS, HTMX |
| `render_content_card()` | Carte contenu reutilisable |
| `render_library_section()` | Fragment complet de la bibliotheque |
| `render_home_platform_section()` | Fragment accueil avec onglets + cartes |
| `render_save_panel()` | Fragment HTMX du bouton favoris |
| `render_auth_page()` | Page inscription/connexion |
| `render_error_page()` | Page d'erreur simple |

Les sorties utilisateur sont echappees avec :

- `html_escape::encode_text` pour le texte ;
- `html_escape::encode_double_quoted_attribute` pour les attributs.

## Acces donnees

Le code utilise des requetes SQLx dynamiques avec `query` et `query_as`.

Fonctions de lecture/ecriture principales :

| Fonction | Role |
| --- | --- |
| `get_contents()` | Liste les contenus, optionnellement filtres par plateforme |
| `get_content_by_slug()` | Charge une fiche contenu |
| `get_content_by_id()` | Charge un contenu par id |
| `get_similar_contents()` | Charge deux contenus proches par competence |
| `is_saved()` | Verifie un favori |
| `saved_contents()` | Liste les favoris d'un utilisateur |
| `history_contents()` | Liste l'historique utilisateur |

## Assets

Les assets sont dans `static/`.

```text
static/app.css       # styles globaux responsive
static/htmx.min.js   # HTMX vendore
static/img/*.svg     # illustrations locales des contenus
```

Les images sont referencees en base par `image_url`, par exemple :

```text
/static/img/bluey.svg
```

## Pages fonctionnelles

### Accueil

Route :

```text
GET /
```

Contient :

- hero ;
- filtre plateforme HTMX ;
- selection du moment ;
- bande statistique ;
- section "Comment ca fonctionne".

### Bibliotheque

Route :

```text
GET /bibliotheque?platform=all
```

Contient :

- filtres YouTube, Netflix, Disney+, Tous ;
- grille de cartes ;
- URLs synchronisees avec `hx-push-url`.

### Fiche contenu

Route :

```text
GET /contenu/{slug}
```

Contient :

- resume contenu ;
- competence developpee ;
- bouton sauvegarde ;
- bouton de redirection plateforme ;
- contenus similaires.

### Compte parent

Route :

```text
GET /compte
```

Si non connecte, redirection vers :

```text
/connexion?next=/compte
```

Si connecte, affiche :

- email ;
- contenus sauvegardes ;
- historique de lancement.

## Redirection plateforme et historique

Route :

```text
GET /go/{slug}
```

Comportement :

1. Charge le contenu par slug.
2. Si un utilisateur est connecte, insere une ligne dans `watch_history`.
3. Redirige vers `source_url`.

En developpement, les URLs pointent vers des recherches ou pages publiques YouTube, Netflix ou Disney+.

## Verification manuelle

Compiler :

```bash
cargo check
```

Lancer :

```bash
cargo run
```

Tester les pages :

```bash
curl -i http://127.0.0.1:3000/
curl -i "http://127.0.0.1:3000/bibliotheque?platform=all"
curl -i http://127.0.0.1:3000/contenu/bluey-saison-1
```

Tester creation compte avec cookie :

```bash
rm -f /tmp/kroissant-cookies.txt
curl -i -c /tmp/kroissant-cookies.txt \
  -X POST http://127.0.0.1:3000/inscription \
  -d "email=dev@example.com&password=password123&next=/compte"

curl -i -b /tmp/kroissant-cookies.txt http://127.0.0.1:3000/compte
```

Tester sauvegarde HTMX cote serveur :

```bash
curl -i -b /tmp/kroissant-cookies.txt \
  -X POST http://127.0.0.1:3000/contenu/1/save
```

## Limites connues

- Tout le code applicatif est dans `src/main.rs`. C'est acceptable pour un prototype, mais a decouper si le projet grandit.
- Les migrations sont inline et idempotentes, sans versionnement.
- Il n'y a pas encore de tests automatises Rust.
- Pas de CSRF token explicite.
- Pas de roles utilisateur.
- Pas de pagination sur la bibliotheque.
- Les contenus sont des donnees fictives seedes localement.
- Le design est code en CSS simple, sans systeme de composants dedie.

## Evolutions recommandees

Priorite technique :

1. Decouper `src/main.rs` en modules : `routes`, `db`, `auth`, `views`, `models`.
2. Ajouter des migrations versionnees avec `sqlx::migrate!`.
3. Ajouter des tests d'integration sur inscription, connexion, favoris et filtres HTMX.
4. Ajouter une protection CSRF pour les formulaires `POST`.
5. Ajouter une vraie configuration d'environnement par fichier `.env`.
6. Ajouter pagination, recherche et tags developpementaux.
7. Ajouter un mode admin pour gerer les contenus.

## Etat actuel

L'application est lancee en developpement sur :

```text
http://127.0.0.1:3000
```

La base locale contient les contenus seedes et peut etre regeneree automatiquement si elle est supprimee.
