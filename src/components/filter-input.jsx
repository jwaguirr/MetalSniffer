import React, { useEffect, useState, useCallback, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";

const FILTER_KEYWORDS = {
  'src': {
    type: 'host',
    description: 'Source address',
    template: 'src host ',
    examples: ['192.168.1.1', '10.0.0.1']
  },
  'dst': {
    type: 'host',
    description: 'Destination address',
    template: 'dst host ',
    examples: ['192.168.1.1', '10.0.0.1']
  },
  'host': {
    type: 'host',
    description: 'Source or destination host',
    template: 'host ',
    examples: ['192.168.1.1']
  },
  'port': {
    type: 'port',
    description: 'Port number',
    template: 'port ',
    examples: ['80', '443', '22']
  },
  'proto': {
    type: 'protocol',
    description: 'Protocol',
    template: '',
    examples: ['tcp', 'udp', 'icmp']
  },
  'tcp': {
    type: 'protocol',
    description: 'TCP protocol',
    template: 'tcp',
    examples: []
  },
  'udp': {
    type: 'protocol',
    description: 'UDP protocol',
    template: 'udp',
    examples: []
  }
};

export const FilterInput = ({ onFilterChange, disabled }) => {
  const [filterText, setFilterText] = useState('');
  const [error, setError] = useState('');
  const [suggestions, setSuggestions] = useState([]);
  const [activeSuggestion, setActiveSuggestion] = useState(-1);
  const [showSuggestions, setShowSuggestions] = useState(false);

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      await invoke("set_packet_filter", { filter: filterText });
      setError('');
      onFilterChange(filterText);
    } catch (err) {
      setError(err.toString());
      console.error('Filter error:', err);
    }
  };

  const getSuggestions = useCallback((text) => {
    const lastWord = text.split(' ').pop().toLowerCase();
    if (lastWord.length === 0) return [];

    return Object.entries(FILTER_KEYWORDS)
      .filter(([keyword]) => keyword.startsWith(lastWord))
      .map(([keyword, data]) => ({
        keyword,
        ...data,
        fullText: text.slice(0, -lastWord.length) + data.template + (data.examples[0] || '')
      }));
  }, []);

  const handleInputChange = (e) => {
    const value = e.target.value;
    setFilterText(value);
    
    const newSuggestions = getSuggestions(value);
    setSuggestions(newSuggestions);
    setShowSuggestions(newSuggestions.length > 0);
    setActiveSuggestion(-1);
  };

  const handleKeyDown = (e) => {
    if (!showSuggestions) return;

    switch (e.key) {
      case 'ArrowDown':
        e.preventDefault();
        setActiveSuggestion(prev => 
          prev < suggestions.length - 1 ? prev + 1 : prev
        );
        break;
      case 'ArrowUp':
        e.preventDefault();
        setActiveSuggestion(prev => prev > 0 ? prev - 1 : -1);
        break;
      case 'Tab':
      case 'Enter':
        if (activeSuggestion >= 0) {
          e.preventDefault();
          setFilterText(suggestions[activeSuggestion].fullText);
          setShowSuggestions(false);
        }
        break;
      case 'Escape':
        setShowSuggestions(false);
        break;
    }
  };

  const handleSuggestionClick = (suggestion) => {
    setFilterText(suggestion.fullText);
    setShowSuggestions(false);
  };

  return (
    <div className="w-full max-w-md">
      <form onSubmit={handleSubmit} className="flex flex-col gap-2">
        <div className="relative">
          <input
            type="text"
            value={filterText}
            onChange={handleInputChange}
            onKeyDown={handleKeyDown}
            placeholder='Enter filter (e.g. "src host 192.168.1.100" or "tcp")'
            className="w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            disabled={disabled}
          />
          <button
            type="submit"
            className="absolute right-2 top-1/2 -translate-y-1/2 px-4 py-1 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:bg-gray-400"
            disabled={disabled}
          >
            Apply
          </button>
          
          {showSuggestions && (
            <div className="absolute w-full mt-1 bg-white border border-gray-300 rounded-md shadow-lg z-10">
              {suggestions.map((suggestion, index) => (
                <div
                  key={suggestion.keyword}
                  className={`px-4 py-2 cursor-pointer hover:bg-gray-100 ${
                    index === activeSuggestion ? 'bg-gray-100' : ''
                  }`}
                  onClick={() => handleSuggestionClick(suggestion)}
                >
                  <div className="font-medium">{suggestion.keyword}</div>
                  <div className="text-sm text-gray-600">{suggestion.description}</div>
                  {suggestion.examples.length > 0 && (
                    <div className="text-xs text-gray-500">
                      Example: {suggestion.examples.join(', ')}
                    </div>
                  )}
                </div>
              ))}
            </div>
          )}
        </div>
        {error && (
          <div className="text-red-500 text-sm">{error}</div>
        )}
      </form>
    </div>
  );
};

export default FilterInput;