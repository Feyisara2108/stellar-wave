'use client';

import React from 'react';
import TimelineWidget from '../../components/timeline';

export default function ClaimFlow() {
  const stream = {
    id: "921",
    total: 1000,
    claimed: 210,
    vested: 450,
  }

  const claimable = stream.vested - stream.claimed;

  return (
    <div className="min-h-screen bg-neutral-950 text-neutral-100 p-8 flex flex-col items-center pt-24">
       <div className="w-full max-w-2xl">
         <h1 className="text-4xl font-bold bg-gradient-to-r from-emerald-400 to-cyan-400 bg-clip-text text-transparent mb-12 text-center">Your Active Stream</h1>
         
         <div className="bg-neutral-900 border border-neutral-800 rounded-2xl p-8 shadow-2xl relative overflow-hidden">
            {/* Background artifact */}
            <div className="absolute -top-32 -right-32 w-64 h-64 bg-emerald-500/10 rounded-full blur-3xl pointer-events-none"></div>
            
            <div className="grid grid-cols-2 gap-8 mb-8">
               <div>
                  <p className="text-neutral-400 text-sm mb-1">Total Vested</p>
                  <p className="text-3xl font-mono font-bold">{stream.vested} <span className="text-sm text-neutral-500">USDC</span></p>
               </div>
               <div className="text-right">
                  <p className="text-neutral-400 text-sm mb-1">Total Claimable</p>
                  <p className="text-3xl font-mono font-bold text-emerald-400">{claimable} <span className="text-sm text-emerald-900">USDC</span></p>
               </div>
            </div>

            <TimelineWidget vested={stream.vested} total={stream.total} claimed={stream.claimed} />

            <div className="mt-12 text-center">
              <button className="px-12 py-4 bg-emerald-500 hover:bg-emerald-600 rounded-full text-neutral-950 font-bold text-xl shadow-[0_0_20px_rgba(16,185,129,0.3)] transition-all active:scale-95">Claim {claimable} USDC</button>
            </div>
         </div>
       </div>
    </div>
  )
}
