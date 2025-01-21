import { useState, useCallback, useEffect } from 'react';

export const useFilteredData = (packetData, isSwitch) => {
  const [filteredData, setFilteredData] = useState({});
  const [filterActiveFilteredData, setFilterActiveFilteredData] = useState({});
  const [currentGroupView, setCurrentGroupView] = useState([]);

  useEffect(() => {
    if (isSwitch) {
      const newFilteredData = {};
      packetData.forEach((packet) => {
        if (packet && packet["Layer 1"]) {
          const tup = [packet["Layer 1"]["Source IP"], packet["Layer 1"]["Destination IP"]].toString();
          if (!newFilteredData[tup]) {
            newFilteredData[tup] = new Set();
          }
          newFilteredData[tup].add(packet);
        }
      });
      setFilteredData(newFilteredData);
    } else {
      setFilteredData({});
    }
  }, [isSwitch, packetData]);

  const viewGroup = useCallback((dest_str, currentFilter, filterActiveFilteredData) => {
    if (currentFilter.length > 0) {
      if (filterActiveFilteredData[dest_str]) {
        setCurrentGroupView(filterActiveFilteredData[dest_str]);
      }
    } else {
      if (filteredData[dest_str]) {
        setCurrentGroupView(filteredData[dest_str]);
      }
    }
  }, [filteredData]);

  const resetGroup = useCallback(() => {
    setCurrentGroupView([]);
  }, []);

  return {
    filteredData,
    filterActiveFilteredData,
    currentGroupView,
    setFilterActiveFilteredData,
    viewGroup,
    resetGroup
  };
};