import React from 'react';

export const Header = ({ isSwitch, handleModeSwitch, packetCount, isActive, toggleCapture }) => (
  <div className="sticky top-0 w-full flex py-4 bg-gray-100 bg-opacity-95 backdrop-blur z-10">
    <div className="container mx-auto px-4 flex justify-between items-center">
      <button 
        onClick={handleModeSwitch}
        className="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300"
      >
        Easy Filtered Mode {isSwitch ? "On" : "Off"}
      </button>
      <h1 className="text-2xl font-bold">
        Packet Data {packetCount > 0 && `(${packetCount})`}
      </h1>
      <button
        onClick={toggleCapture}
        className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
      >
        {isActive ? "Stop Packet Capture" : "Start Packet Capture"}
      </button>
    </div>
  </div>
);