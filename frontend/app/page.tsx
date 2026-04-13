import React from 'react';

export default function DashboardPage() {
  return (
    <div className="min-h-screen bg-neutral-950 text-neutral-100 p-8">
      <header className="mb-12 flex justify-between items-center border-b border-neutral-800 pb-4">
        <h1 className="text-3xl font-bold bg-gradient-to-r from-blue-400 to-emerald-400 bg-clip-text text-transparent">Stellar-Wave Dashboard</h1>
        <nav className="flex gap-4">
            <a href="/create" className="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-md font-medium transition-colors">Create Stream</a>
            <a href="/claim" className="px-4 py-2 border border-neutral-700 hover:bg-neutral-800 rounded-md font-medium transition-colors">Claim Flow</a>
            <a href="/admin" className="px-4 py-2 border border-neutral-700 hover:bg-neutral-800 rounded-md font-medium transition-colors">Admin Panel</a>
        </nav>
      </header>

      <main className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        <section className="p-6 bg-neutral-900 border border-neutral-800 rounded-xl shadow-lg flex flex-col items-center justify-center min-h-[200px]">
           <h2 className="text-xl text-neutral-400 mb-2">Total Value Locked</h2>
           <p className="text-5xl font-mono font-bold">$1.2M</p>
        </section>

        <section className="p-6 bg-neutral-900 border border-neutral-800 rounded-xl shadow-lg flex flex-col items-center justify-center min-h-[200px]">
           <h2 className="text-xl text-neutral-400 mb-2">Active Streams</h2>
           <p className="text-5xl font-mono font-bold text-blue-400">3,492</p>
        </section>

        <section className="p-6 bg-neutral-900 border border-neutral-800 rounded-xl shadow-lg flex flex-col items-center justify-center min-h-[200px]">
           <h2 className="text-xl text-neutral-400 mb-2">Total Claimed</h2>
           <p className="text-5xl font-mono font-bold text-emerald-400">840K</p>
        </section>
      </main>

      <div className="mt-12">
         <h2 className="text-2xl font-semibold mb-6 border-b border-neutral-800 pb-2">Recent Streams</h2>
         <div className="overflow-x-auto">
            <table className="w-full text-left bg-neutral-900 rounded-lg overflow-hidden border border-neutral-800">
               <thead className="bg-neutral-800">
                  <tr>
                     <th className="p-4 font-semibold text-neutral-300">Recipient</th>
                     <th className="p-4 font-semibold text-neutral-300">Amount</th>
                     <th className="p-4 font-semibold text-neutral-300">Progress</th>
                     <th className="p-4 font-semibold text-neutral-300">Status</th>
                  </tr>
               </thead>
               <tbody className="divide-y divide-neutral-800">
                  {[1, 2, 3, 4, 5].map(i => (
                     <tr key={i} className="hover:bg-neutral-800/50 transition-colors">
                        <td className="p-4 font-mono text-sm text-neutral-400">GABC...{1000 + i}</td>
                        <td className="p-4 font-mono font-medium">{i * 100} USDC</td>
                        <td className="p-4">
                           <div className="w-full bg-neutral-700 rounded-full h-2.5">
                              <div className="bg-emerald-400 h-2.5 rounded-full" style={{ width: `${i * 15}%`}}></div>
                           </div>
                        </td>
                        <td className="p-4"><span className="px-2 py-1 bg-blue-500/20 text-blue-400 rounded text-xs font-semibold">Streaming</span></td>
                     </tr>
                  ))}
               </tbody>
            </table>
         </div>
      </div>
    </div>
  );
}
