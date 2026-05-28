"use client";

import { useState } from "react";
import Link from "next/link";

export default function ParentDashboardPage() {
  // États simulés pour le MVP
  const [quietMode, setQuietMode] = useState(true);
  const [ageLimit, setAgeLimit] = useState(4);

  return (
    <main className="max-w-[1140px] mx-auto p-6 md:p-8">
      {/* En-tête */}
      <header className="mb-10">
        <Link href="/" className="text-primary font-bold flex items-center gap-2 mb-6 hover:opacity-80 transition-opacity w-fit">
          <span>←</span> Retour à l'accueil
        </Link>
        <h1 className="text-[32px] md:text-[48px] font-bold text-on-surface leading-tight">
          Espace Parent
        </h1>
        <p className="text-[16px] md:text-[18px] text-on-surface-variant font-medium mt-2">
          Gérez l'expérience de votre enfant en toute sérénité.
        </p>
      </header>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        
        {/* Colonne Principale (Statistiques & Réglages) */}
        <div className="lg:col-span-2 flex flex-col gap-8">
          
          {/* Carte : Temps d'écran */}
          <section className="bg-white rounded-2xl border border-surface-container-highest p-6 shadow-sm">
            <h2 className="text-[20px] font-bold text-on-surface mb-6 flex items-center gap-2">
              <span className="text-secondary">⏱</span> Temps de visionnage
            </h2>
            
            <div className="mb-2 flex justify-between items-end">
              <div>
                <span className="text-[32px] font-bold text-on-surface leading-none">25</span>
                <span className="text-on-surface-variant font-medium ml-1">min aujourd'hui</span>
              </div>
              <span className="text-[14px] font-bold text-on-surface-variant">Limite : 45 min</span>
            </div>
            
            {/* Barre de progression (Spécification DESIGN.md : Track = Secondary, Indicator = Primary) */}
            <div className="h-2 w-full bg-secondary-container rounded-full overflow-hidden mb-4">
              <div 
                className="h-full bg-primary rounded-full transition-all duration-500 ease-out"
                style={{ width: `${(25 / 45) * 100}%` }}
              />
            </div>
            <p className="text-[14px] text-on-surface-variant font-medium">
              Le contenu se mettra en pause doucement une fois la limite atteinte.
            </p>
          </section>

          {/* Carte : Filtres & Sécurité */}
          <section className="bg-white rounded-2xl border border-surface-container-highest p-6 shadow-sm">
            <h2 className="text-[20px] font-bold text-on-surface mb-6 flex items-center gap-2">
              <span className="text-primary">🛡</span> Filtres de Contenu
            </h2>

            <div className="flex flex-col gap-6">
              {/* Toggle Mode Calme */}
              <div className="flex items-center justify-between p-4 bg-surface rounded-xl">
                <div>
                  <h3 className="font-bold text-on-surface">Quiet Mode strict</h3>
                  <p className="text-[14px] text-on-surface-variant font-medium mt-1">
                    Masque automatiquement tout contenu ayant un badge "Stimulant".
                  </p>
                </div>
                <button 
                  onClick={() => setQuietMode(!quietMode)}
                  className={`w-14 h-8 rounded-full p-1 transition-colors duration-300 ${
                    quietMode ? 'bg-tertiary' : 'bg-surface-container-high'
                  }`}
                >
                  <div className={`w-6 h-6 bg-white rounded-full shadow-sm transform transition-transform duration-300 ${
                    quietMode ? 'translate-x-6' : 'translate-x-0'
                  }`} />
                </button>
              </div>

              {/* Réglage de l'âge */}
              <div className="flex items-center justify-between p-4 bg-surface rounded-xl">
                <div>
                  <h3 className="font-bold text-on-surface">Niveau de développement</h3>
                  <p className="text-[14px] text-on-surface-variant font-medium mt-1">
                    Âge configuré : {ageLimit} ans
                  </p>
                </div>
                <Link 
                  href="/onboarding"
                  className="px-4 py-2 border-2 border-secondary-container text-secondary font-bold rounded-lg hover:bg-secondary-container/20 transition-colors"
                >
                  Modifier
                </Link>
              </div>
            </div>
          </section>

        </div>

        {/* Colonne Latérale (Historique / Info) */}
        <div className="flex flex-col gap-8">
          
          {/* Carte : Historique Récent */}
          <section className="bg-white rounded-2xl border border-surface-container-highest p-6 shadow-sm">
            <h2 className="text-[20px] font-bold text-on-surface mb-6">Vu récemment</h2>
            
            <div className="flex flex-col gap-4">
              {/* Item Historique 1 */}
              <div className="flex items-center gap-4 group cursor-pointer">
                <div className="w-16 h-12 bg-surface-container-highest rounded-md overflow-hidden relative flex-shrink-0">
                  <div className="absolute inset-0 bg-gradient-to-br from-secondary-container to-surface-container" />
                </div>
                <div>
                  <h4 className="font-bold text-[14px] text-on-surface group-hover:text-primary transition-colors leading-tight">
                    Peinture Magique
                  </h4>
                  <p className="text-[12px] text-on-surface-variant font-medium mt-0.5">
                    Il y a 2 heures
                  </p>
                </div>
              </div>

              {/* Item Historique 2 */}
              <div className="flex items-center gap-4 group cursor-pointer">
                <div className="w-16 h-12 bg-surface-container-highest rounded-md overflow-hidden relative flex-shrink-0">
                  <div className="absolute inset-0 bg-gradient-to-br from-surface-dim to-surface-container" />
                </div>
                <div>
                  <h4 className="font-bold text-[14px] text-on-surface group-hover:text-primary transition-colors leading-tight">
                    Promenade en Forêt
                  </h4>
                  <p className="text-[12px] text-on-surface-variant font-medium mt-0.5">
                    Hier
                  </p>
                </div>
              </div>
            </div>

            <button className="w-full mt-6 py-3 text-[14px] font-bold text-secondary bg-secondary-container/30 hover:bg-secondary-container/50 rounded-xl transition-colors">
              Voir tout l'historique
            </button>
          </section>

          {/* Carte : Déconnexion / Compte */}
          <section className="bg-surface-container rounded-2xl p-6 text-center">
            <div className="w-12 h-12 bg-surface-container-highest rounded-full flex items-center justify-center mx-auto mb-4 text-on-surface-variant">
              👤
            </div>
            <p className="font-bold text-on-surface mb-1">parent@exemple.fr</p>
            <p className="text-[12px] text-on-surface-variant font-medium mb-6">Abonnement Premium Actif</p>
            
            <button className="text-on-surface-variant hover:text-primary font-bold text-[14px] transition-colors underline decoration-2 underline-offset-4">
              Se déconnecter
            </button>
          </section>

        </div>
      </div>
    </main>
  );
}