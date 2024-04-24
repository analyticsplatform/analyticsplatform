'use client'

// components/Chatbot.tsx

import React, { useState } from 'react';

const Chatbot: React.FC = () => {
  const [messages, setMessages] = useState<string[]>([]);
  const [inputMessage, setInputMessage] = useState('');

  const handleSendMessage = () => {
    if (inputMessage.trim() !== '') {
      setMessages([...messages, inputMessage]);
      setInputMessage('');
    }
  };

  return (
    <div className="bg-white shadow-md rounded-md p-4 flex flex-col h-full">
      <div className="flex-grow overflow-y-auto mb-4">
        {messages.map((message, index) => (
          <div key={index} className="mb-2">
            <p className="text-gray-600">{message}</p>
          </div>
        ))}
      </div>
      <div className="flex">
        <input
          type="text"
          className="flex-grow border border-gray-300 rounded-l-md px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="Type your message..."
          value={inputMessage}
          onChange={(e) => setInputMessage(e.target.value)}
        />
        <button
          className="bg-blue-500 text-white px-4 py-2 rounded-r-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
          onClick={handleSendMessage}
        >
          Send
        </button>
      </div>
    </div>
  );
};

export default Chatbot;
