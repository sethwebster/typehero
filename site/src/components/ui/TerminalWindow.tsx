"use client";

import { useState, useEffect } from "react";
import { motion } from "framer-motion";
import { DEMO_CODE } from "@/lib/constants";

export default function TerminalWindow() {
  const [text, setText] = useState("");
  const [showStats, setShowStats] = useState(false);

  useEffect(() => {
    let timeout: NodeJS.Timeout;

    // Typing simulation logic
    const type = (index: number) => {
      if (index <= DEMO_CODE.length) {
        setText(DEMO_CODE.slice(0, index));
        if (index === DEMO_CODE.length) {
          setShowStats(true);
          timeout = setTimeout(() => {
            setShowStats(false);
            setText("");
            type(0);
          }, 5000);
        } else {
          // Variable typing speed for realism
          timeout = setTimeout(() => type(index + 1), Math.random() * 30 + 30);
        }
      }
    };

    timeout = setTimeout(() => type(0), 1000);
    return () => clearTimeout(timeout);
  }, []);

  return (
    <div className="w-full max-w-5xl overflow-hidden rounded-xl border border-white/10 bg-[#0c0c0c] shadow-2xl backdrop-blur-sm mx-auto">
      {/* Terminal Title Bar */}
      <div className="flex items-center justify-between border-b border-white/5 bg-[#1a1a1a] px-4 py-3">
        <div className="flex gap-2">
          <div className="h-3 w-3 rounded-full bg-[#ff5f56] border border-[#e0443e]" />
          <div className="h-3 w-3 rounded-full bg-[#ffbd2e] border border-[#dea123]" />
          <div className="h-3 w-3 rounded-full bg-[#27c93f] border border-[#1aab29]" />
        </div>
        <div className="font-mono text-xs text-zinc-500 flex items-center gap-2">
          <span className="hidden sm:inline">typehero — split — 120x24</span>
        </div>
        <div className="w-16" /> {/* Spacer for centering */}
      </div>

      {/* Split Pane Layout */}
      <div className="flex flex-col md:flex-row min-h-[400px] font-mono text-sm md:text-[15px] leading-relaxed">

        {/* Left Pane: Source Code (Reference) */}
        <div className="flex-1 border-r border-white/5 bg-[#0c0c0c]/50 p-6 relative group overflow-hidden">

          {/* Pane Label */}
          <div className="absolute top-0 right-0 p-2 opacity-50">
            <span className="text-[10px] uppercase tracking-wider text-zinc-600 border border-zinc-800 px-2 py-1 rounded">Reference</span>
          </div>

          <div className="flex flex-col gap-[2px]">
            {/* 
                  Hardcoded Syntax Highlighting for robustness. 
                  Matches DEMO_CODE lines.
                */}
            <div className="flex"><span className="w-6 mr-4 text-right text-zinc-700 select-none">1</span><span><span className="text-[#c678dd]">function</span> <span className="text-[#61afef]">calculateSpeed</span><span className="text-zinc-500">(</span><span className="text-[#e06c75]">wpm</span><span className="text-zinc-500">,</span> <span className="text-[#e06c75]">accuracy</span><span className="text-zinc-500">)</span> <span className="text-zinc-500">{`{`}</span></span></div>

            <div className="flex"><span className="w-6 mr-4 text-right text-zinc-700 select-none">2</span><span>  <span className="text-[#c678dd]">if</span> <span className="text-zinc-500">(</span><span className="text-[#e06c75]">accuracy</span> &lt; <span className="text-[#d19a66]">90</span><span className="text-zinc-500">)</span> <span className="text-[#c678dd]">return</span> <span className="text-[#98c379]">"Practice more!"</span><span className="text-zinc-500">;</span></span></div>

            <div className="flex"><span className="w-6 mr-4 text-right text-zinc-700 select-none">3</span><span>  <span className="text-[#c678dd]">return</span> <span className="text-[#98c379]">{`\`You are typing at `}</span><span className="text-[#e06c75]">{`\${wpm}`}</span><span className="text-[#98c379]">{` WPM!\``}</span><span className="text-zinc-500">;</span></span></div>

            <div className="flex"><span className="w-6 mr-4 text-right text-zinc-700 select-none">4</span><span><span className="text-zinc-500">{`}`}</span></span></div>

            <div className="flex"><span className="w-6 mr-4 text-right text-zinc-700 select-none">5</span><span></span></div>

            <div className="flex"><span className="w-6 mr-4 text-right text-zinc-700 select-none">6</span><span className="text-[#5c6370] italic">// Start your journey today</span></div>

            <div className="flex"><span className="w-6 mr-4 text-right text-zinc-700 select-none">7</span><span><span className="text-[#c678dd]">const</span> <span className="text-[#e06c75]">hero</span> <span className="text-[#56b6c2]">=</span> <span className="text-[#c678dd]">new</span> <span className="text-[#e5c07b]">TypeHero</span><span className="text-zinc-500">();</span></span></div>

            <div className="flex"><span className="w-6 mr-4 text-right text-zinc-700 select-none">8</span><span><span className="text-[#e06c75]">hero</span><span className="text-zinc-500">.</span><span className="text-[#61afef]">train</span><span className="text-zinc-500">();</span></span></div>
          </div>
        </div>

        {/* Right Pane: Active Terminal (User Input) */}
        <div className="flex-1 bg-[#0c0c0c] p-6 relative">
          <div className="absolute top-0 right-0 p-2 opacity-50">
            <span className="text-[10px] uppercase tracking-wider text-green-900 border border-green-900/30 text-green-500 px-2 py-1 rounded">Interactive</span>
          </div>

          <div className="text-zinc-300">
            <div className="mb-4 text-green-500 font-bold flex items-center gap-2">
              <span>➜</span>
              <span className="text-[#61afef]">~</span>
              <span>typehero practice</span>
            </div>

            {/* Typing Output with Cursor */}
            <div className="whitespace-pre-wrap relative break-all font-medium text-gray-200">
              {text}
              <motion.span
                animate={{ opacity: [1, 0] }}
                transition={{ repeat: Infinity, duration: 0.8 }}
                className="inline-block h-5 w-2.5 translate-y-1 bg-green-500 ml-0.5 align-middle shadow-[0_0_8px_rgba(34,197,94,0.8)]"
              />
            </div>
          </div>

          {/* Stats Overlay */}
          {showStats && (
            <div className="absolute inset-0 z-10 flex items-center justify-center bg-black/60 backdrop-blur-[2px]">
              <motion.div
                initial={{ opacity: 0, scale: 0.9 }}
                animate={{ opacity: 1, scale: 1 }}
                className="bg-[#1a1a1a] border border-white/10 rounded-xl p-6 shadow-2xl max-w-xs w-full mx-4"
              >
                <h3 className="text-lg font-bold text-white mb-4 flex items-center gap-2">
                  <span className="text-xl">🎉</span> Session Complete
                </h3>
                <div className="space-y-4">
                  <div className="flex justify-between items-center border-b border-white/5 pb-2">
                    <span className="text-zinc-400">Words Per Minute</span>
                    <span className="text-xl font-mono font-bold text-green-400">92</span>
                  </div>
                  <div className="flex justify-between items-center border-b border-white/5 pb-2">
                    <span className="text-zinc-400">Accuracy</span>
                    <span className="text-xl font-mono font-bold text-blue-400">100%</span>
                  </div>
                  <div className="pt-2">
                    <div className="w-full bg-blue-600 hover:bg-blue-500 text-white text-center py-2 rounded-lg text-sm font-bold cursor-pointer transition-colors">
                      Play Again
                    </div>
                  </div>
                </div>
              </motion.div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
