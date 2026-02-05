import Link from "next/link";
import { Terminal } from "lucide-react";

export function Header() {
  return (
    <header className="fixed top-0 z-50 w-full border-b border-white/5 bg-black/50 backdrop-blur-xl">
      <div className="container mx-auto flex h-16 items-center justify-between px-6">
        <Link href="/" className="flex items-center gap-2 text-lg font-bold tracking-tight text-white hover:opacity-80 transition-opacity">
          <Terminal className="h-5 w-5 text-green-500" />
          <span>TypeHero</span>
        </Link>
        <nav className="hidden md:flex items-center gap-8 text-sm font-medium text-zinc-400">
          <Link href="#features" className="hover:text-white transition-colors">Features</Link>
          <Link href="#how-it-works" className="hover:text-white transition-colors">How it works</Link>
          <Link href="https://github.com/sethwebster/typehero" target="_blank" className="hover:text-white transition-colors">GitHub</Link>
        </nav>
        <div className="flex items-center gap-4">
          <Link
            href="https://github.com/sethwebster/typehero"
            target="_blank"
            className="hidden sm:inline-flex h-9 items-center justify-center rounded-full bg-white px-4 text-sm font-medium text-black transition-colors hover:bg-zinc-200"
          >
            Star on GitHub
          </Link>
        </div>
      </div>
    </header>
  );
}
