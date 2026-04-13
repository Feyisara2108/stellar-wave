'use client';

import React, { useState } from 'react';

export default function CreateStream() {
  const [recipient, setRecipient] = useState('');
  const [amount, setAmount] = useState('');
  
  const handleCreate = (e: React.FormEvent) => {
    e.preventDefault();
    console.log("Creating stream for", recipient, amount);
  }

  return (
    <div className="min-h-screen bg-neutral-950 text-neutral-100 p-8 flex flex-col items-center justify-center">
      <div className="w-full max-w-lg bg-neutral-900 border border-neutral-800 rounded-2xl shadow-xl p-8">
        <h2 className="text-3xl font-bold bg-gradient-to-r from-blue-400 to-indigo-400 bg-clip-text text-transparent mb-6">Create Stream</h2>
        <form onSubmit={handleCreate} className="space-y-6">
          
          <div>
             <label className="block text-sm font-medium text-neutral-400 mb-2">Recipient Public Key</label>
             <input type="text" value={recipient} onChange={e => setRecipient(e.target.value)} placeholder="G..." className="w-full bg-neutral-950 border border-neutral-800 rounded-lg p-3 text-neutral-200 outline-none focus:border-blue-500 transition-colors font-mono" required />
          </div>

          <div>
             <label className="block text-sm font-medium text-neutral-400 mb-2">Total Amount (Token)</label>
             <input type="number" step="0.0000001" value={amount} onChange={e => setAmount(e.target.value)} placeholder="0.00" className="w-full bg-neutral-950 border border-neutral-800 rounded-lg p-3 text-neutral-200 outline-none focus:border-blue-500 transition-colors font-mono" required />
          </div>

          <div className="grid grid-cols-2 gap-4">
             <div>
                <label className="block text-sm font-medium text-neutral-400 mb-2">Start Ledger</label>
                <input type="number" placeholder="Now" className="w-full bg-neutral-950 border border-neutral-800 rounded-lg p-3 text-neutral-200 outline-none focus:border-blue-500 transition-colors font-mono" />
             </div>
             <div>
                <label className="block text-sm font-medium text-neutral-400 mb-2">End Ledger</label>
                <input type="number" placeholder="End" className="w-full bg-neutral-950 border border-neutral-800 rounded-lg p-3 text-neutral-200 outline-none focus:border-blue-500 transition-colors font-mono" required />
             </div>
          </div>

          <div>
             <label className="block text-sm font-medium text-neutral-400 mb-2">Cliff Ledger (Optional)</label>
             <input type="number" placeholder="Cliff" className="w-full bg-neutral-950 border border-neutral-800 rounded-lg p-3 text-neutral-200 outline-none focus:border-blue-500 transition-colors font-mono" />
          </div>

          <button type="submit" className="w-full py-4 rounded-lg bg-blue-600 hover:bg-blue-700 text-white font-semibold text-lg transition-transform active:scale-[0.98]">Broadcast to Soroban</button>

        </form>
      </div>
    </div>
  )
}
