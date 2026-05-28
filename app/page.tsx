"use client";

import { VideoProps } from '@/types';
import { useFilters } from '../hooks/useFilters';
import { VideoCard } from '../src/components/feature/VideoCard';
import { VIDEOS_DATA } from '@/src/data/videos';



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