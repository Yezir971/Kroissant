import Link from 'next/link';

export interface VideoProps {
  id: string;
  titre: string;
  description_courte: string;
  age_minimum: number;
  duree_minutes: number;
  slow_score: number;
  badge_cognitif: "Modéré" | "Stimulant" | "Doux" | "Très Doux" | "Apaisant";
  plateforme: string;
}

export const VideoCard = ({ video }: { video: VideoProps }) => {
  return (
    <article className="group cursor-pointer">
      <Link href={`/video/${video.id}`} className="block">
        {/* Thumbnail Area */}
        <div className="relative aspect-video bg-surface-container-high rounded-xl overflow-hidden mb-4 shadow-sm group-hover:shadow-md transition-all">
          {/* Placeholder image (à remplacer par une vraie balise img) */}
          <div className="absolute inset-0 bg-gradient-to-br from-secondary-container/40 to-surface-container" />
          
          {/* Score Badge superposé */}
          <div className="absolute top-3 right-3 bg-surface/90 backdrop-blur-sm px-3 py-1.5 rounded-full text-xs font-bold text-on-surface shadow-sm">
            ⚲ {video.slow_score}/10
          </div>
        </div>

        {/* Content Area */}
        <h3 className="text-xl font-bold text-on-surface leading-snug mb-2">
          {video.titre}
        </h3>
        
        <div className="flex items-center gap-3">
          {/* Badge Cognitif (Pillule bleue) */}
          <span className="bg-secondary-container text-on-secondary-container px-3 py-1 rounded-full text-xs font-bold">
            {video.badge_cognitif}
          </span>
          <span className="text-xs font-medium text-on-surface-variant">
            {video.plateforme} • {video.duree_minutes} min
          </span>
        </div>
      </Link>
    </article>
  );
};