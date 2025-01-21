import React from 'react';

export const GroupView = ({ currentGroupView, resetGroup }) => {
  if (currentGroupView.size === 0) return null;
console.log(currentGroupView)
  const firstPacket = Array.from(currentGroupView)[0];

  return currentGroupView.size > 0 && (
    <div className="mt-2 w-full flex justify-center items-center relative">
      <div className="flex flex-col justify-center w-32">
        <h1>Group Packet</h1>
        <h1 className="truncate">
          {firstPacket?.["Layer 1"]?.["Source IP"] || "N/A"}
        </h1>
        <h1 className="truncate">
          {firstPacket?.["Layer 1"]?.["Destination IP"] || "N/A"}
        </h1>
      </div>
      <button 
        onClick={resetGroup} 
        className="rounded-md bg-slate-800 py-2 px-4 border border-transparent text-center text-sm text-white transition-all shadow-md hover:shadow-lg focus:bg-slate-700 focus:shadow-none active:bg-slate-700 hover:bg-slate-700 active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none ml-2 absolute left-0 top-4"
        type="button"
      >
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="size-6">
          <path strokeLinecap="round" strokeLinejoin="round" d="M9 15 3 9m0 0 6-6M3 9h12a6 6 0 0 1 0 12h-3" />
        </svg>
      </button>
    </div>
  );
};
