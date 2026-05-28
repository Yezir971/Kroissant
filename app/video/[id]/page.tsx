import { notFound } from "next/navigation";
import Link from "next/link";
import { VIDEOS_DATA } from "@/src/data/videos";

// 1. Le composant devient asynchrone
// 2. Le typage de params devient une Promise
export default async function VideoDetailPage({ 
  params 
}: { 
  params: Promise<{ id: string }> 
}) {
  // 3. On "unwrap" la promesse pour récupérer l'ID
  const { id } = await params;
  
  // 4. On utilise l'ID extrait
  const video = VIDEOS_DATA.find((v) => v.id === id);

  if (!video) {
    notFound();
  }

  return (
    <main className="max-w-4xl mx-auto p-6 md:py-12">
      <Link href="/" className="text-indigo-600 font-bold flex items-center gap-2 mb-8 group">
        <span className="group-hover:-translate-x-1 transition-transform">←</span> Retour à la sélection
      </Link>

      <div className="bg-white rounded-3xl p-8 md:p-12 shadow-sm border border-slate-100">
        <header className="mb-8">
          <div className="flex gap-2 mb-4">
             <span className="px-3 py-1 bg-slate-100 rounded-full text-xs font-bold uppercase">{video.plateforme}</span>
             <span className="px-3 py-1 bg-indigo-50 text-indigo-700 rounded-full text-xs font-bold">{video.age_minimum}+ ans</span>
          </div>
          <h1 className="text-4xl font-extrabold text-slate-900 mb-4 leading-tight">
            {video.titre}
          </h1>
          <p className="text-xl text-slate-600 leading-relaxed">
            {video.description_courte}
          </p>
        </header>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-10">
          <div className="p-4 bg-slate-50 rounded-2xl border border-slate-100">
            <p className="text-xs text-slate-400 uppercase font-bold mb-1">Rythme</p>
            <p className="text-lg font-bold">Score: {video.slow_score}/10</p>
          </div>
          <div className="p-4 bg-slate-50 rounded-2xl border border-slate-100">
            <p className="text-xs text-slate-400 uppercase font-bold mb-1">Éveil</p>
            <p className="text-lg font-bold">{video.badge_cognitif}</p>
          </div>
          <div className="p-4 bg-slate-50 rounded-2xl border border-slate-100">
            <p className="text-xs text-slate-400 uppercase font-bold mb-1">Durée</p>
            <p className="text-lg font-bold">{video.duree_minutes} minutes</p>
          </div>
        </div>

        <div className="border-t pt-10">
          <h2 className="font-bold mb-4">Pourquoi nous recommandons ce contenu :</h2>
          <p className="text-slate-600 mb-8">
            Ce contenu a été analysé pour son rythme calme ({video.slow_score}) 
            et son aspect {video.badge_cognitif.toLowerCase()}. Idéal pour une consommation 
            réfléchie sans surexcitation.
          </p>

          <a
            href={video.url_externe}
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center justify-center w-full md:w-auto px-10 py-4 bg-indigo-600 text-white font-black rounded-2xl text-lg hover:bg-indigo-700 transition-colors shadow-xl shadow-indigo-100"
          >
            Regarder sur {video.plateforme} ↗
          </a>
          <p className="text-center md:text-left text-xs text-slate-400 mt-4">
            Vous allez être redirigé vers une plateforme externe sécurisée.
          </p>
        </div>
      </div>
    </main>
  );
}