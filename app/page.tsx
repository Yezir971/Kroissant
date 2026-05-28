"use client";

import { useFilters } from '../hooks/useFilters';
import { VideoCard, VideoProps } from '../src/components/feature/VideoCard';

// Injection des données fournies (Dans une V2, ceci proviendra d'une API/BDD)
const VIDEOS_DATA: VideoProps[] = [
  { "id": "v1", "titre": "L'Ours et la Lune", "description_courte": "Une douce balade nocturne pour découvrir les constellations en poésie.", "age_minimum": 3, "duree_minutes": 10, "nombre_de_plans": 40, "slow_score": 10.0, "badge_cognitif": "Modéré", "plateforme": "France TV", "url_externe": "https://www.france.tv/l-ours-et-la-lune" },
  { "id": "v2", "titre": "Aventures Super Rapides", "description_courte": "Des courses effrénées dans la ville des voitures colorées.", "age_minimum": 5, "duree_minutes": 5, "nombre_de_plans": 100, "slow_score": 0.8, "badge_cognitif": "Stimulant", "plateforme": "YouTube", "url_externe": "https://www.youtube.com/watch?v=mock1" },
  { "id": "v3", "titre": "Le Jardin de Papi", "description_courte": "Apprendre à faire pousser des tomates avec patience et douceur.", "age_minimum": 4, "duree_minutes": 7, "nombre_de_plans": 42, "slow_score": 6.2, "badge_cognitif": "Modéré", "plateforme": "Netflix", "url_externe": "https://www.netflix.com/title/mock3" },
  { "id": "v4", "titre": "Zou le Zèbre", "description_courte": "Zou explore la savane et rencontre de nouveaux amis bondissants.", "age_minimum": 3, "duree_minutes": 15, "nombre_de_plans": 300, "slow_score": 0.8, "badge_cognitif": "Stimulant", "plateforme": "Disney+", "url_externe": "https://www.disneyplus.com/video/mock4" },
  { "id": "v5", "titre": "La forêt des murmures", "description_courte": "Un documentaire animé sur la vie cachée des insectes en automne.", "age_minimum": 6, "duree_minutes": 22, "nombre_de_plans": 110, "slow_score": 7.7, "badge_cognitif": "Modéré", "plateforme": "Arte", "url_externe": "https://www.arte.tv/fr/videos/mock5" },
  { "id": "v6", "titre": "Mission Galactique", "description_courte": "Des héros spatiaux sauvent la galaxie à bord de leur vaisseau hyper-vitesse.", "age_minimum": 7, "duree_minutes": 5, "nombre_de_plans": 40, "slow_score": 4.2, "badge_cognitif": "Stimulant", "plateforme": "Netflix", "url_externe": "https://www.netflix.com/title/mock6" },
  { "id": "v7", "titre": "Peinture sur nuages", "description_courte": "Atelier créatif et contemplatif dans les cieux imaginaires.", "age_minimum": 4, "duree_minutes": 12, "nombre_de_plans": 60, "slow_score": 7.7, "badge_cognitif": "Modéré", "plateforme": "YouTube", "url_externe": "https://www.youtube.com/watch?v=mock7" },
  { "id": "v8", "titre": "Le Défi des Toupie-Batailles", "description_courte": "Un tournoi de toupies magiques avec des effets visuels explosifs.", "age_minimum": 6, "duree_minutes": 8, "nombre_de_plans": 160, "slow_score": 0.8, "badge_cognitif": "Stimulant", "plateforme": "YouTube", "url_externe": "https://www.youtube.com/watch?v=mock8" },
  { "id": "v9", "titre": "L'Océan Profond", "description_courte": "Plongée immersive et silencieuse à la rencontre des baleines bleues.", "age_minimum": 5, "duree_minutes": 25, "nombre_de_plans": 150, "slow_score": 6.2, "badge_cognitif": "Modéré", "plateforme": "Disney+", "url_externe": "https://www.disneyplus.com/video/mock9" },
  { "id": "v10", "titre": "Les formes s'endorment", "description_courte": "Des formes géométriques qui s'emboîtent lentement pour aider à la sieste.", "age_minimum": 2, "duree_minutes": 4, "nombre_de_plans": 15, "slow_score": 10.0, "badge_cognitif": "Modéré", "plateforme": "France TV", "url_externe": "https://www.france.tv/les-formes-s-endorment" }
];

export default function HomePage() {
  const { 
    filteredData, 
    activeFilter, 
    setActiveFilter, 
    availableOptions 
  } = useFilters<VideoProps>(VIDEOS_DATA, 'plateforme');

  return (
    <main className="min-h-screen bg-slate-50 p-6 md:p-12">
      <div className="max-w-7xl mx-auto">
        <header className="mb-10">
          <h1 className="text-3xl font-extrabold text-slate-900 mb-6">
            Découvertes adaptées
          </h1>
          
          {/* Barre de filtres (pourrait être extraite dans un composant FilterBar) */}
          <div className="flex flex-wrap gap-3">
            <button
              onClick={() => setActiveFilter(null)}
              className={`px-4 py-2 rounded-full text-sm font-semibold transition-colors ${
                activeFilter === null 
                  ? 'bg-slate-800 text-white' 
                  : 'bg-white text-slate-600 hover:bg-slate-100 border border-slate-200'
              }`}
            >
              Toutes
            </button>
            
            {availableOptions.map((plateforme) => (
              <button
                key={plateforme}
                onClick={() => setActiveFilter(plateforme)}
                className={`px-4 py-2 rounded-full text-sm font-semibold transition-colors ${
                  activeFilter === plateforme 
                    ? 'bg-indigo-600 text-white' 
                    : 'bg-white text-slate-600 hover:bg-slate-100 border border-slate-200'
                }`}
              >
                {plateforme}
              </button>
            ))}
          </div>
        </header>

        {/* Grille de vidéos */}
        <section className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
          {filteredData.map((video) => (
            <VideoCard key={video.id} video={video} />
          ))}
        </section>

        {filteredData.length === 0 && (
          <div className="text-center py-20 text-slate-500">
            Aucune vidéo trouvée pour ce filtre.
          </div>
        )}
      </div>
    </main>
  );
}