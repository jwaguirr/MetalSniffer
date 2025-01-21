import React from 'react';

export const PacketCard = React.memo(({ packet, onClick, packet_no }) => {
  if (!packet || !packet["Layer 1"]) return null;
  
  return (
    <div
      onClick={onClick}
      className="max-w-64 flex flex-col space-y-2 h-full border-2 p-2 border-black rounded-lg bg-white shadow-lg hover:shadow-xl transition-shadow cursor-pointer"
    >
      <div className="text-center">
        {packet["Layer 1"].Version} : No.{packet_no}
      </div>
      <div className="truncate">
        Source IP: {packet["Layer 1"]["Source IP"]}
      </div>
      <div className="truncate">
        Destination IP: {packet["Layer 1"]["Destination IP"]}
      </div>
      <div>
        Protocol: {packet["Layer 2"]?.Protocol || "N/A"}
      </div>
      <div className="text-center">
        {packet.timestamp?.timestamp || "N/A"}
      </div>
    </div>
  );
}, (prevProps, nextProps) => {
  return prevProps.packet?.timestamp?.timestamp === nextProps.packet?.timestamp?.timestamp &&
         prevProps.packet_no === nextProps.packet_no;
});