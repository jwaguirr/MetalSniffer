import React, { useState, useCallback, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";
import ResizableFooter from "./components/resizable-footer";
import { FilterInput } from "./components/filter-input";
import { usePacketCapture } from "./hooks/usePacketCapture";
import { useFilteredData } from "./hooks/useFilteredData";
import { PacketCard } from "./components/packetCard";
import { EasyPacketCard } from "./components/easyPacketCard";
import { Header } from "./components/header";
import { GroupView } from "./components/groupView";

function App() {
  const [isSwitch, setIsSwitch] = useState(false);
  const [currentPacket, setCurrentPacket] = useState(null);
  const [currentFilter, setCurrentFilter] = useState('');

  const {
    packetData,
    isActive,
    filterActiveData,
    toggleCapture,
    setPacketData,
    setFilterActiveData
  } = usePacketCapture(currentFilter);

  const {
    filteredData,
    filterActiveFilteredData,
    currentGroupView,
    setFilterActiveFilteredData,
    viewGroup,
    resetGroup
  } = useFilteredData(packetData, isSwitch);

  const handleFilterChange = useCallback(async (newFilter) => {
    setCurrentFilter(newFilter);
    if (newFilter.trim().length <= 0) {
      setFilterActiveData([]);
      setFilterActiveFilteredData({});
    }

    if (isActive) {
      await invoke("stop_packet_capture");
      setPacketData([]);
      await invoke("start_packet_capture");
    }

    if (newFilter.trim()) {
      const filteredPackets = await invoke("filter_packets", {
        packets: packetData,
        filter: newFilter
      });
      setFilterActiveData(filteredPackets);
      
      const newFilteredFilteredData = {};
      filteredPackets.forEach((packet) => {
        if (packet && packet["Layer 1"]) {
          const tup = [packet["Layer 1"]["Source IP"], packet["Layer 1"]["Destination IP"]].toString();
          if (!newFilteredFilteredData[tup]) {
            newFilteredFilteredData[tup] = new Set();
          }
          newFilteredFilteredData[tup].add(packet);
        }
      });
      setFilterActiveFilteredData(newFilteredFilteredData);
    }
  }, [isActive, packetData]);

  const handleModeSwitch = useCallback(() => {
    setIsSwitch(prev => !prev);
    resetGroup();
    setCurrentPacket(null);
  }, [resetGroup]);

  const toggleCard = useCallback((packet) => {
    setCurrentPacket(prev => prev === packet ? null : packet);
  }, []);

  const visiblePackets = useMemo(() => {
    if (isSwitch) {
      return currentGroupView.size > 0 ? Array.from(currentGroupView) : [];
    }
    return currentFilter.length > 0 ? filterActiveData : packetData;
  }, [packetData, filterActiveData, isSwitch, currentGroupView, currentFilter]);

  return (
    <div className="flex flex-col min-h-screen bg-gray-100">
      <div className="flex-grow p-8">
        <Header 
          isSwitch={isSwitch}
          handleModeSwitch={handleModeSwitch}
          packetCount={packetData.length}
          isActive={isActive}
          toggleCapture={toggleCapture}
        />

        <div className="flex justify-center mx-auto px-4 mt-4">
          <FilterInput 
            onFilterChange={handleFilterChange}
            disabled={isActive}
          />
          {currentFilter && (
            <div className="mt-2 text-sm text-gray-600">
              Current filter: {currentFilter}
            </div>
          )}
        </div>
          {
            isSwitch && (
              <GroupView 
                currentGroupView={currentGroupView}
                resetGroup={resetGroup}
              />
            )
          }

        <div className="mt-12 overflow-auto">
          <div className="flex flex-row flex-wrap gap-4 justify-center">
            {!isSwitch ? (
              visiblePackets.map((packet, index) => (
                <PacketCard
                  key={`${packet?.timestamp?.timestamp}-${index}`}
                  packet={packet}
                  onClick={() => toggleCard(packet)}
                  packet_no={packetData.length - index - 1}
                />
              ))
            ) : (
              currentGroupView?.size > 0 ? (
                Array.from(currentGroupView).map((packet, index) => (
                  <PacketCard
                    key={`${packet?.timestamp?.timestamp}-${index}`}
                    packet={packet}
                    onClick={() => toggleCard(packet)}
                    packet_no={packetData.length - index - 1}
                  />
                ))
              ) : (
                Object.entries(currentFilter.length > 0 ? filterActiveFilteredData : filteredData).map(([key], index) => (
                  <EasyPacketCard 
                    key={`${key}-${index}`}
                    src_dest_str={key}
                    onClick={() => viewGroup(key, currentFilter, filterActiveFilteredData)}
                  />
                ))
              )
            )}
          </div>
        </div>
      </div>
      <ResizableFooter packetData={currentPacket} />
    </div>
  );
}

export default App;