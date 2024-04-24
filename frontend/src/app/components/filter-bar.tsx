// app/dashboard/components/filter-bar.tsx

'use client';

import React, { useState } from 'react';

const FilterBar: React.FC = () => {
  const [selectedTopic, setSelectedTopic] = useState('');
  const [selectedStartDate, setSelectedStartDate] = useState(new Date());
  const [selectedEndDate, setSelectedEndDate] = useState(new Date());

  const handleTopicChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    setSelectedTopic(event.target.value);
  };

  const handleStartDateChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setSelectedStartDate(new Date(event.target.value));
  };

  const handleEndDateChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setSelectedEndDate(new Date(event.target.value));
  };

  return (
    <div className="mb-4">
      <div className="flex flex-col md:flex-row md:justify-between md:items-center">
        <div className="flex flex-col md:flex-row md:items-center mb-4 md:mb-0">
          <select
            className="px-4 py-2 border border-gray-300 rounded-md mt-2 md:mt-0 md:ml-4"
            value={selectedTopic}
            onChange={handleTopicChange}
          >
            <option value="">Select Topic</option>
            <option value="topic1">Topic 1</option>
            <option value="topic2">Topic 2</option>
            <option value="topic3">Topic 3</option>
          </select>
        </div>
        <div className="flex flex-col md:flex-row md:items-center">
          <div className="flex items-center mb-2 md:mb-0">
            <input
              type="date"
              className="px-4 py-2 border border-gray-300 rounded-md mr-2"
              value={selectedStartDate.toISOString().slice(0, 10)}
              onChange={handleStartDateChange}
            />
            <span className="mr-2">to</span>
            <input
              type="date"
              className="px-4 py-2 border border-gray-300 rounded-md"
              value={selectedEndDate.toISOString().slice(0, 10)}
              onChange={handleEndDateChange}
            />
          </div>
          <button disabled={true} className="px-4 py-2 bg-gray-500 text-white rounded-md mt-2 md:mt-0 md:ml-4">
            Export
          </button>
        </div>
      </div>
    </div>
  );
};

export default FilterBar;
