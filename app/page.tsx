"use client";

import { useFilters } from '@/hooks/useFilters';
import { VideoCard } from '@/src/components/feature/VideoCard';
import { VIDEOS_DATA } from '@/src/data/videos';
import { VideoProps } from '@/types';

export default function HomePage() {
  const { 
    filteredData, 
    activeFilter, 
    setActiveFilter, 
    availableOptions 
  } = useFilters<VideoProps>(VIDEOS_DATA, 'plateforme');

  return (
    <main className="max-w-[1140px] mx-auto p-6 md:p-8">
      <header className="mb-12 max-w-2xl">
        <h1 className="text-[32px] md:text-[48px] font-bold text-on-surface mb-4 leading-tight tracking-tight">
          Découvertes Douces
        </h1>
        <p className="text-[16px] md:text-[18px] text-on-surface-variant font-medium leading-relaxed">
          Une sélection de contenus apaisants, vérifiés pour leur rythme lent et leur faible impact cognitif.
        </p>
      </header>

      {/* Barre de filtres (Chips) */}
      <div className="flex flex-wrap gap-3 mb-10">
        <button
          onClick={() => setActiveFilter(null)}
          className={`px-5 py-2 rounded-full text-sm font-semibold transition-all ${
            activeFilter === null 
              ? 'bg-primary text-on-primary' 
              : 'bg-surface-container hover:bg-surface-container-high text-on-surface-variant'
          }`}
        >
          Tous les contenus
        </button>
        
        {availableOptions.map((plateforme) => (
          <button
            key={plateforme}
            onClick={() => setActiveFilter(plateforme)}
            className={`px-5 py-2 rounded-full text-sm font-semibold transition-all ${
              activeFilter === plateforme 
                ? 'bg-primary text-on-primary' 
                : 'bg-surface-container hover:bg-surface-container-high text-on-surface-variant'
            }`}
          >
            {plateforme}
          </button>
        ))}
      </div>

      {/* Grille fluide de vidéos */}
      <section className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
        {filteredData.map((video) => (
          <VideoCard key={video.id} video={video} />
        ))}
      </section>
    </main>
  );
}