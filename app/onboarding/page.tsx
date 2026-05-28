"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";

export default function OnboardingPage() {
  const [age, setAge] = useState<number>(4);
  const router = useRouter();

  const handleStart = () => {
    localStorage.setItem("child_age", age.toString());
    router.push("/");
  };

  return (
    <main className="flex-grow flex flex-col items-center justify-center p-6 bg-surface">
      <div className="w-full max-w-md flex flex-col items-center">
        
        {/* Barre de progression */}
        <div className="w-full mb-12">
          <p className="text-[12px] font-bold text-primary uppercase tracking-widest mb-2">Étape 2 sur 3</p>
          <div className="h-2 w-full bg-surface-container-high rounded-full overflow-hidden">
            <div className="h-full bg-primary w-2/3 rounded-full" />
          </div>
        </div>

        {/* Icone et Titre */}
        <div className="w-12 h-12 bg-surface-container-high rounded-full flex items-center justify-center mb-6 text-primary">
          ☺
        </div>
        <h1 className="text-[32px] font-bold text-on-surface text-center mb-4 leading-tight">
          Quel âge a votre enfant ?
        </h1>
        <p className="text-center text-on-surface-variant font-medium mb-10">
          Cela nous aide à personnaliser un contenu apaisant et adapté à son stade de développement.
        </p>

        {/* Carte du Slider */}
        <div className="w-full bg-white rounded-3xl p-8 mb-8 shadow-sm">
          <div className="flex justify-center mb-8">
            <div className="w-32 h-32 bg-primary-container/30 rounded-full flex items-center justify-center">
              <span className="text-[48px] font-bold text-primary leading-none">{age}</span>
              <span className="text-[16px] text-primary font-medium ml-1 mt-4">ans</span>
            </div>
          </div>

          {/* Slider input */}
          <div className="relative pt-4 pb-2">
            <input 
              type="range" 
              min="1" 
              max="10" 
              value={age}
              onChange={(e) => setAge(parseInt(e.target.value))}
              className="w-full h-2 bg-surface-container-high rounded-lg appearance-none cursor-pointer accent-primary"
            />
            <div className="flex justify-between mt-4 text-[14px] font-semibold text-on-surface-variant">
              <span>1 an</span>
              <span>10 ans</span>
            </div>
          </div>
        </div>

        {/* Actions */}
        <button
          onClick={handleStart}
          className="w-full bg-primary text-on-primary font-bold py-4 rounded-xl text-[18px] hover:bg-on-primary-container transition-colors mb-4"
        >
          Continuer →
        </button>
        <button className="text-on-surface-variant font-semibold text-[14px] hover:text-primary transition-colors">
          Retour
        </button>
      </div>
    </main>
  );
}