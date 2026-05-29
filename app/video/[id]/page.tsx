import { VIDEOS_DATA } from "@/src/data/videos";

export default async function VideoDetailPage({ 
  params 
}: { 
  params: Promise<{ id: string }> 
}) {
  const { id } = await params;
  const video = VIDEOS_DATA.find((v) => v.id === id);


  return (
    <main className="max-w-[1140px] mx-auto p-6 md:p-8">
      <div className="flex flex-col lg:flex-row gap-12 items-start mt-8">
        
        {/* Lecteur vidéo (Gauche) */}
        <div className="w-full lg:w-[60%] aspect-video bg-surface-container-high rounded-[1.5rem] relative flex items-center justify-center overflow-hidden shadow-sm">
          {/* Placeholder visuel du lecteur */}
          <div className="absolute inset-0 bg-gradient-to-br from-surface-dim to-surface-container" />
          <button className="relative z-10 w-16 h-16 bg-primary/80 text-on-primary rounded-full flex items-center justify-center hover:bg-primary transition-colors shadow-lg">
            ▶
          </button>
        </div>

        {/* Informations et Actions (Droite) */}
        <div className="w-full lg:w-[40%] flex flex-col pt-4">
          <h1 className="text-[32px] md:text-[48px] font-bold text-on-surface leading-tight mb-4">
            {video?.titre}
          </h1>
          
          <p className="text-[16px] text-on-surface-variant leading-relaxed font-medium mb-8">
            {video?.description_courte} Une histoire conçue pour apaiser et préparer au calme, sans transitions brusques ni bruits forts.
          </p>

          {/* Badges de score (Pillules bleues) */}
          <div className="flex gap-4 mb-10">
            <div className="bg-secondary-container px-4 py-2 rounded-full flex flex-col">
              <span className="text-[10px] uppercase font-bold text-on-secondary-container/70 tracking-wider">Slow Score</span>
              <span className="text-on-secondary-container font-bold">{video?.slow_score}/10</span>
            </div>
            <div className="bg-secondary-container px-4 py-2 rounded-full flex flex-col">
              <span className="text-[10px] uppercase font-bold text-on-secondary-container/70 tracking-wider">Impact Cognitif</span>
              <span className="text-on-secondary-container font-bold capitalize">{video?.badge_cognitif}</span>
            </div>
          </div>

          {/* Call to Action principal */}
          <a
            href={video?.url_externe}
            target="_blank"
            rel="noopener noreferrer"
            className="w-full bg-primary text-on-primary text-center py-4 rounded-xl font-bold text-[18px] hover:bg-on-primary-container transition-colors shadow-sm"
          >
            ▶ Voir le dessin animé
          </a>
          <p className="text-center text-[12px] font-bold text-on-surface-variant/60 mt-3 uppercase tracking-wider">
            S'ouvre dans une nouvelle fenêtre sécurisée
          </p>
        </div>
        
      </div>
    </main>
  );
}