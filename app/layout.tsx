import type { Metadata } from "next";
import { Quicksand } from "next/font/google";
import "./globals.css";

const quicksand = Quicksand({ 
  subsets: ["latin"],
  variable: "--font-quicksand",
});

export const metadata: Metadata = {
  title: "QuietTime",
  description: "Un environnement numérique apaisant.",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="fr">
      <body className={`${quicksand.variable} font-sans bg-surface text-on-surface min-h-screen flex flex-col`}>
        {/* Navigation inspirée de screen1.jpg */}
        <nav className="h-20 px-8 flex items-center justify-between max-w-[1140px] mx-auto w-full">
          <div className="text-primary font-bold text-2xl tracking-tight">
            QuietTime
          </div>
          <div className="hidden md:flex gap-8 text-on-surface-variant font-semibold text-sm">
            <span className="text-primary border-b-2 border-primary pb-1">Home</span>
            <span className="hover:text-primary cursor-pointer transition-colors">Library</span>
            <span className="hover:text-primary cursor-pointer transition-colors">Quiet Mode</span>
            <span className="hover:text-primary cursor-pointer transition-colors">Favorites</span>
          </div>
          <div className="flex items-center gap-4">
            <button className="bg-primary text-on-primary px-4 py-2 rounded-full text-sm font-bold">
              Parent Dashboard
            </button>
          </div>
        </nav>
        <div className="flex-grow">
          {children}
        </div>
      </body>
    </html>
  );
}