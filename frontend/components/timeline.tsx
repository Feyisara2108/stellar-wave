import React from 'react';

export default function TimelineWidget({ vested, total, claimed }: { vested: number, total: number, claimed: number }) {
  const vestedPercent = Math.min(100, Math.max(0, (vested / total) * 100));
  const claimedPercent = Math.min(100, Math.max(0, (claimed / total) * 100));

  return (
    <div className="w-full relative mt-8 mb-4">
      {/* Background track */}
      <div className="w-full h-4 bg-neutral-800 rounded-full overflow-hidden absolute top-0 left-0"></div>
      
      {/* Vested Track */}
      <div 
        className="h-4 bg-blue-500/50 rounded-full absolute top-0 left-0 transition-all duration-1000 ease-out"
        style={{ width: \`\${vestedPercent}%\` }}
      ></div>

      {/* Claimed Track */}
      <div 
        className="h-4 bg-emerald-500 rounded-full absolute top-0 left-0 transition-all duration-1000 ease-out z-10"
        style={{ width: \`\${claimedPercent}%\` }}
      ></div>

      <div className="flex justify-between mt-6 text-sm">
        <span className="text-neutral-500 font-mono">Start</span>
        <span className="text-neutral-500 font-mono">End</span>
      </div>
    </div>
  )
}
