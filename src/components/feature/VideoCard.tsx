import { VideoProps } from '@/types';
import Link from 'next/link';


export const VideoCard = ({ video }: { video: VideoProps }) => {
  const isModere = video.badge_cognitif === "Modéré";

  return (
    <article className="flex flex-col bg-white rounded-2xl shadow-sm border border-slate-100 overflow-hidden hover:shadow-md transition-shadow">
      <div className="p-5 flex flex-col h-full gap-3">
        {/* En-tête : Plateforme et Âge */}
        <div className="flex justify-between items-center text-xs font-semibold text-slate-500">
          <span className="uppercase tracking-wider">{video.plateforme}</span>
          <span className="bg-slate-100 px-2 py-1 rounded-full">{video.age_minimum}+ ans</span>
        </div>

        {/* Contenu principal */}
        <div className="flex-grow">
          <h3 className="text-lg font-bold text-slate-800 leading-tight mb-2">
            {video.titre}
          </h3>
          <p className="text-sm text-slate-600 line-clamp-2">
            {video.description_courte}
          </p>
        </div>

        {/* Métriques : Slow Score & Badge Cognitif */}
        <div className="flex items-center gap-2 mt-2">
          <span className="text-xs font-medium bg-indigo-50 text-indigo-700 px-2 py-1 rounded-md border border-indigo-100">
            Slow Score: {video.slow_score.toFixed(1)}
          </span>
          <span 
            className={`text-xs font-medium px-2 py-1 rounded-md border ${
              isModere 
                ? 'bg-emerald-50 text-emerald-700 border-emerald-100' 
                : 'bg-amber-50 text-amber-700 border-amber-100'
            }`}
          >
            {video.badge_cognitif}
          </span>
        </div>

        {/* Action */}
        <Link 
          href={`/video/${video.id}`}
          className="mt-4 w-full block text-center bg-indigo-600 text-white font-semibold py-2.5 rounded-xl hover:bg-indigo-700 transition-colors"
        >
          Voir les détails
        </Link>
      </div>
    </article>
  );
};