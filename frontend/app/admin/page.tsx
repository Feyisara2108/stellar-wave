'use client';

import React from 'react';

export default function AdminPanel() {
  return (
    <div className="min-h-screen bg-neutral-950 text-neutral-100 p-8">
      <header className="mb-12 border-b border-neutral-800 pb-4">
        <h1 className="text-3xl font-bold text-red-400">Admin Control Panel</h1>
      </header>

      <section className="mb-12">
        <div className="bg-neutral-900 border border-red-900/50 rounded-xl p-6 shadow-xl w-full max-w-xl">
           <h2 className="text-xl font-semibold mb-2">Protocol Global State</h2>
           <p className="text-neutral-400 mb-6">Pause or unpause the entire streaming protocol. When paused, claims and creations are halted.</p>
           
           <button className="px-6 py-3 bg-red-600 hover:bg-red-700 font-bold rounded-lg transition-colors w-full">EMERGENCY PAUSE</button>
        </div>
      </section>

      <section>
        <h2 className="text-xl font-semibold mb-4">Stream Management</h2>
        <div className="overflow-x-auto">
            <table className="w-full text-left bg-neutral-900 rounded-lg overflow-hidden border border-neutral-800">
               <thead className="bg-neutral-800">
                  <tr>
                     <th className="p-4 font-semibold text-neutral-300">ID</th>
                     <th className="p-4 font-semibold text-neutral-300">Recipient</th>
                     <th className="p-4 font-semibold text-neutral-300">Amount</th>
                     <th className="p-4 font-semibold text-neutral-300">Action</th>
                  </tr>
               </thead>
               <tbody className="divide-y divide-neutral-800">
                  {[1, 2, 3].map(i => (
                     <tr key={i} className="hover:bg-neutral-800/50 transition-colors">
                        <td className="p-4 font-mono text-neutral-400">{i}</td>
                        <td className="p-4 font-mono text-sm text-neutral-400">GABC...{1000 + i}</td>
                        <td className="p-4 font-mono font-medium">{i * 100} USDC</td>
                        <td className="p-4"><button className="px-3 py-1 bg-neutral-800 hover:bg-red-900 text-red-500 rounded text-sm font-semibold transition-colors">Cancel Stream</button></td>
                     </tr>
                  ))}
               </tbody>
            </table>
         </div>
      </section>
    </div>
  )
}
