export interface VideoProps {
  id: string;
  titre: string;
  description_courte: string;
  age_minimum: number;
  duree_minutes: number;
  slow_score: number;
  nombre_de_plans: number,
  badge_cognitif: "Modéré" | "Stimulant";
  plateforme: string;
  url_externe: string
}