// components/Newsfeed.tsx

import React from 'react';

const Feed: React.FC = () => {
  const items = [...Array(15)].map((_, i) => {
    return { id: i, title: `Feed Item ${i}`, description: `Description of news item ${i}` }
  });

  return (
    <div className="bg-white shadow-md rounded-md p-4 h-full divide-y-2 divide-sky-50 overflow-y-auto">
      {items.map((item) => (
        <div key={item.id} className="mb-4 pt-4">
          <h3 className="text-lg font-bold">{item.title}</h3>
          <p className="text-gray-600">{item.description}</p>
        </div>
      ))}
    </div>
  );
};

export default Feed;
