import React from 'react';

export const EasyPacketCard = React.memo(({ src_dest_str, onClick }) => {
  if (!src_dest_str) return null;
  const src_dest_arr = src_dest_str.split(",");
  
  return (
    <div
      onClick={onClick}
      className="max-w-64 flex flex-col space-y-2 h-full border-2 p-2 border-black rounded-lg bg-white shadow-lg hover:shadow-xl transition-shadow cursor-pointer"
    >
      <div className="truncate">
        Source IP: {src_dest_arr[0] || "N/A"}
      </div>
      <div className="truncate">
        Destination IP: {src_dest_arr[1] || "N/A"}
      </div>
    </div>
  );
}, (prevProps, nextProps) => {
  return prevProps.src_dest_str === nextProps.src_dest_str;
});