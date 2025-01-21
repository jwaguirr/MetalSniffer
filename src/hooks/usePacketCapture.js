import { useState, useCallback, useEffect } from 'react';
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export const usePacketCapture = (currentFilter) => {
  const [packetData, setPacketData] = useState([]);
  const [isActive, setIsActive] = useState(false);
  const [filterActiveData, setFilterActiveData] = useState([]);

  const handleNewPacket = useCallback((newPacket) => {
    if (!newPacket || !newPacket["Layer 1"]) {
      console.error("Invalid packet data received");
      return;
    }

    if (currentFilter.length > 0) {
      setFilterActiveData(prevData => [newPacket, ...prevData]);
    } else {
      setPacketData(prevData => [newPacket, ...prevData]);
    }
  }, [currentFilter]);

  const toggleCapture = useCallback(async () => {
    if (isActive) {
      await invoke("stop_packet_capture");
    } else {
      await invoke("start_packet_capture");
    }
    setIsActive(prev => !prev);
  }, [isActive]);

  useEffect(() => {
    let isSubscribed = true;
    const setupListener = async () => {
      const unlisten = await listen("packet-captured", (event) => {
        if (isSubscribed) {
          handleNewPacket(event.payload);
        }
      });
      
      return () => {
        isSubscribed = false;
        unlisten();
      };
    };

    setupListener();
  }, [handleNewPacket]);

  return {
    packetData,
    isActive,
    filterActiveData,
    toggleCapture,
    setPacketData,
    setFilterActiveData
  };
};