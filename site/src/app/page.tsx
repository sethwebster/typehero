"use client";

import { motion } from "framer-motion";
import { Zap, Trophy, Code } from "lucide-react";
import { Header } from "@/components/layout/Header";
import TerminalWindow from "@/components/ui/TerminalWindow";

export default function Home() {
  const features = [
    {
      icon: <Code className="h-6 w-6 text-blue-500" />,
      title: "Real Code Snippets",
      description: "Practice typing with actual code from popular open-source repositories. Support for JavaScript, Python, Rust, Go, and more."
    },
    {
      icon: <Zap className="h-6 w-6 text-yellow-500" />,
      title: "Instant Feedback",
      description: "Get real-time metrics on your typing speed (WPM) and accuracy. Identify weak keys and improve your muscle memory."
    },
    {
      icon: <Trophy className="h-6 w-6 text-green-500" />,
      title: "Gamified Mastery",
      description: "Track your progress over time. Earn achievements, climb the leaderboard, and become a true keyboard warrior."
    }
  ];

  return (
    <div className="min-h-screen bg-black text-white selection:bg-green-500/30 selection:text-green-500 overflow-x-hidden">
      <Header />

      {/* Background Gradients */}
      <div className="fixed inset-0 z-0 pointer-events-none">
        <div className="absolute top-[-10%] left-[-10%] h-[500px] w-[500px] rounded-full bg-green-500/10 blur-[120px]" />
        <div className="absolute bottom-[-10%] right-[-10%] h-[500px] w-[500px] rounded-full bg-blue-500/10 blur-[120px]" />
      </div>

      <main className="relative z-10 flex flex-col items-center pt-32 pb-20 px-6">

        {/* Hero Section */}
        <section className="w-full max-w-5xl mx-auto text-center flex flex-col items-center gap-8 mb-32">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5 }}
            className="flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-3 py-1 text-sm text-zinc-400 backdrop-blur-sm"
          >
            <span className="flex h-2 w-2 rounded-full bg-green-500 animate-pulse" />
            v1.0 is now available
          </motion.div>

          <motion.h1
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5, delay: 0.1 }}
            className="text-5xl md:text-7xl font-bold tracking-tight bg-gradient-to-br from-white via-zinc-200 to-zinc-500 bg-clip-text text-transparent max-w-3xl"
          >
            Master the Terminal. <br />
            <span className="text-white">Become a TypeHero.</span>
          </motion.h1>

          <motion.p
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5, delay: 0.2 }}
            className="text-lg md:text-xl text-zinc-400 max-w-2xl leading-relaxed"
          >
            The CLI tool that transforms your terminal into a powerful typing dojo.
            Practice with real code, track your stats, and code faster than ever.
          </motion.p>

          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5, delay: 0.3 }}
            className="flex flex-col sm:flex-row items-center gap-4 mt-4"
          >
            <div className="group relative flex items-center gap-3 rounded-lg bg-zinc-900 pr-4 pl-4 py-3 font-mono text-sm text-zinc-300 border border-zinc-800 transition-colors hover:border-zinc-700">
              <span className="text-green-500">$</span>
              <span>npx @sethwebster/typehero@latest</span>
              <button
                onClick={() => navigator.clipboard.writeText("npx @sethwebster/typehero@latest")}
                className="ml-4 rounded hover:bg-zinc-800 p-1 transition-colors"
                aria-label="Copy command"
              >
                <svg className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                </svg>
              </button>
            </div>
          </motion.div>

          <motion.div
            initial={{ opacity: 0, scale: 0.95 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ duration: 0.7, delay: 0.4 }}
            className="w-full mt-16 relative"
          >
            <div className="absolute -inset-1 bg-gradient-to-r from-green-500 to-blue-500 rounded-2xl blur opacity-20 pointer-events-none" />
            <div className="flex justify-center">
              <TerminalWindow />
            </div>
          </motion.div>
        </section>

        {/* Features Section */}
        <section id="features" className="w-full max-w-6xl mx-auto py-20">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            {features.map((feature, idx) => (
              <motion.div
                key={idx}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.5, delay: idx * 0.1 }}
                viewport={{ once: true }}
                className="group p-8 rounded-2xl border border-white/10 bg-white/5 hover:bg-white/10 transition-colors backdrop-blur-sm"
              >
                <div className="mb-4 inline-flex items-center justify-center rounded-lg bg-white/5 p-3 ring-1 ring-white/10 group-hover:ring-white/20 transition-all">
                  {feature.icon}
                </div>
                <h3 className="text-xl font-bold mb-2 text-white">{feature.title}</h3>
                <p className="text-zinc-400 leading-relaxed">{feature.description}</p>
              </motion.div>
            ))}
          </div>
        </section>

        {/* CTA Section */}
        <section className="w-full max-w-4xl mx-auto py-32 text-center">
          <motion.div
            initial={{ opacity: 0 }}
            whileInView={{ opacity: 1 }}
            transition={{ duration: 0.8 }}
            viewport={{ once: true }}
            className="relative overflow-hidden rounded-3xl bg-gradient-to-b from-zinc-900 to-black border border-white/10 p-12 md:p-20"
          >
            <div className="relative z-10 flex flex-col items-center gap-6">
              <h2 className="text-3xl md:text-4xl font-bold text-white">Ready to boost your speed?</h2>
              <p className="text-zinc-400 max-w-lg mb-6">Join thousands of developers who are mastering their keyboard with TypeHero. Open your terminal and start typing.</p>
              <div className="flex items-center gap-3 rounded-lg bg-black/50 pr-4 pl-4 py-4 font-mono text-lg text-zinc-300 border border-zinc-800">
                <span className="text-green-500">$</span>
                <span>npx typehero@latest</span>
              </div>
            </div>

            {/* Decorative background grid */}
            <div className="absolute inset-0 z-0 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-[size:24px_24px] [mask-image:radial-gradient(ellipse_60%_50%_at_50%_0%,#000_70%,transparent_100%)]" />
          </motion.div>
        </section>

        {/* Footer */}
        <footer className="w-full border-t border-white/10 py-12 text-center text-zinc-500 text-sm">
          <div className="flex items-center justify-center gap-8 mb-8">
            <a href="#" className="hover:text-white transition-colors">Privacy</a>
            <a href="#" className="hover:text-white transition-colors">Terms</a>
            <a href="https://github.com/sethwebster/typehero" className="hover:text-white transition-colors">GitHub</a>
            <a href="#" className="hover:text-white transition-colors">Twitter</a>
          </div>
          <p>© {new Date().getFullYear()} TypeHero. All rights reserved.</p>
        </footer>

      </main>
    </div>
  );
}
