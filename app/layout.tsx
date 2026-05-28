import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "KidVids - Curation Sécurisée",
  description: "Le meilleur contenu pour vos enfants, à leur rythme.",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="fr">
      <body className={`${inter.className} bg-slate-50 text-slate-900`}>
        <nav className="h-16 border-b bg-white flex items-center px-6 justify-between sticky top-0 z-50">
          <span className="font-black text-xl text-indigo-600 tracking-tighter">KIDVIDS</span>
          <div className="h-8 w-8 bg-slate-200 rounded-full" /> {/* Avatar simulé */}
        </nav>
        {children}
      </body>
    </html>
  );
}