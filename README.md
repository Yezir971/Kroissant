# 🥐 Kroissant - Réseau Social

[![Bun](https://img.shields.io/badge/Bun-%23000000.svg?style=for-the-badge&logo=bun&logoColor=white)](https://bun.sh)
[![Elysia](https://img.shields.io/badge/Elysia.js-%2310b981.svg?style=for-the-badge&logo=elysia&logoColor=white)](https://elysiajs.com)
[![React](https://img.shields.io/badge/React-%2320232a.svg?style=for-the-badge&logo=react&logoColor=%2361dafb)](https://react.dev)


**Kroissant** est un réseau social moderne, axé sur la performance, la légèreté et le partage de contenus. Conçu sous forme de monorepo, il tire parti de la puissance de **Bun** comme runtime, d'**Elysia.js** pour une API Backend typée et performante, et de **React** pour une interface utilisateur fluide et réactive.

---

## 📝 Sommaire
1. [Contexte du Projet](#-contexte-du-projet)
2. [Stack Technique](#-stack-technique)
3. [Architecture du Projet](#-architecture-du-projet)
4. [Diagrammes UML](#-diagrammes-uml)
5. [Installation Locale](#-installation-locale)
6. [Flux de Fonctionnalité Principale](#-flux-de-fonctionnalite-principale)
7. [Sécurité & Bonnes Pratiques](#-securite--bonnes-pratiques)
8. [Contribution & Licence](#-contribution--licence)

---

## 🎯 Contexte du Projet

Dans un écosystème web saturé par des applications lourdes et gourmandes en ressources, **Kroissant** est né avec un objectif clair : redéfinir la rapidité d'un réseau social. 

L'application permet aux utilisateurs de créer un profil, de publier des messages (les "Kroissants"), de suivre d'autres utilisateurs, de réagir à des publications en temps réel et d'interagir via une messagerie instantanée. 

### Objectifs majeurs :
- **Performance brute :** Réponses de l'API sous la barre des 10ms grâce à Bun et Elysia.js.
- **Expérience utilisateur (UX) optimale :** Interface SPA (Single Page Application) fluide avec mise à jour optimiste des états.
- **Temps réel natif :** Flux d'actualités et notifications propulsés par WebSockets.

---

## 🛠️ Stack Technique

### Backend & Runtime
- **Runtime :** [Bun](https://bun.sh/) (Alternative ultra-rapide à Node.js)
- **Framework API :** [Elysia.js](https://elysiajs.com/) (Framework web haute performance avec validation de types de bout en bout via Eden)
- **Base de données :** PostgreSQL + Prisma ORM / Drizzle ORM

### Frontend
- **Framework :** [React](https://react.dev/) (avec Vite ou Next.js)
- **Gestion d'état / Requêtes :** TanStack Query (React Query)
- **Styling :** Tailwind CSS

---

## 📂 Architecture du Projet

Le projet est structuré sous forme de **Monorepo** pour faciliter le partage de types entre le Front et le Back.

```bash
kroissant/
src/
├── app/                      # Routage et pages (App Router Next.js 14+)
│   ├── layout.tsx            # Layout principal (Header, Navigation)
│   ├── page.tsx              # Page d'accueil (Grille + Filtres)
│   ├── onboarding/           # Fonctionnalité d'Onboarding
│   │   └── page.tsx          # Création du profil enfant
│   └── video/
│       └── [id]/             # Routage dynamique
│           └── page.tsx      # Page Détail (Résumé + Redirection externe)
├── components/               # Composants UI (Dumb components)
│   ├── ui/                   # Éléments de Design System (Boutons, Badges)
│   └── feature/              # Composants métiers
│       ├── VideoCard.tsx     # Composant d'affichage de la vidéo
│       └── FilterBar.tsx     # Barre de filtres
├── hooks/                    # Logique métier réutilisable (Smart logic)
│   └── useFilters.ts         # Hook de gestion d'état des filtres
├── types/                    # Définitions TypeScript
│   └── index.ts              # Interfaces (ex: Video, Profile)
└── utils/                    # Fonctions utilitaires pures (DRY)
```
