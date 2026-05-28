"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";

export default function OnboardingPage() {
  const [age, setAge] = useState<number>(3);
  const router = useRouter();

  const handleStart = () => {
    // Simulation : on stocke l'âge en local pour le MVP
    localStorage.setItem("child_age", age.toString());
    router.push("/");
  };

  return (
    <main className="min-h-[calc(100vh-64px)] flex items-center justify-center p-6">
      <div className="max-w-md w-full bg-white p-8 rounded-3xl shadow-xl border border-slate-100 text-center">
        <h1 className="text-3xl font-bold mb-2">Bienvenue ! 🎈</h1>
        <p className="text-slate-500 mb-8">Créons le profil de votre enfant pour adapter le contenu.</p>
        
        <div className="mb-10">
          <label className="block text-sm font-bold text-slate-700 mb-4 uppercase tracking-wide">
            Quel âge a l'enfant ?
          </label>
          <div className="flex items-center justify-center gap-6">
            <button 
              onClick={() => setAge(Math.max(2, age - 1))}
              className="h-12 w-12 rounded-full border-2 border-indigo-100 flex items-center justify-center text-2xl hover:bg-indigo-50"
            >
              -
            </button>
            <span className="text-5xl font-black text-indigo-600 w-20">{age}</span>
            <button 
              onClick={() => setAge(Math.min(12, age + 1))}
              className="h-12 w-12 rounded-full border-2 border-indigo-100 flex items-center justify-center text-2xl hover:bg-indigo-50"
            >
              +
            </button>
          </div>
          <p className="text-xs text-slate-400 mt-4 italic">Contenu filtré automatiquement pour cet âge.</p>
        </div>

        <button
          onClick={handleStart}
          className="w-full bg-indigo-600 text-white font-bold py-4 rounded-2xl text-lg hover:scale-[1.02] transition-transform shadow-lg shadow-indigo-200"
        >
          Découvrir les vidéos
        </button>
      </div>
    </main>
  );
}