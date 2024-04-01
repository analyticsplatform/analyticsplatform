"use client"

import { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { ChevronDownIcon } from '@heroicons/react/24/solid';

const CustomAccordion = ({ data_info }) => {
  const [open, setOpen] = useState(false);

  const toggleAccordion = () => {
    setOpen(!open);
  };

  return (
    <div className="w-full mb-4 border border-gray-200 rounded-lg">
      <div
        className="text-blue-950 font-bold flex items-center justify-between p-4 hover:bg-sky-100/30 cursor-pointer rounded-lg"
        onClick={toggleAccordion}
      >
        <div>
          <span className="mr-2">{data_info.name}</span>
          <span className="text-gray-500 text-sm">{data_info.provider}</span>
        </div>
        <motion.div
          animate={{ rotate: open ? 180 : 0 }}
          transition={{ duration: 0.3 }}
        >
          <ChevronDownIcon className="w-5 h-5 text-gray-500" />
        </motion.div>
      </div>
      <AnimatePresence>
        {open && (
          <motion.div
            initial={{ height: 0, opacity: 0 }}
            animate={{ height: 'auto', opacity: 1 }}
            exit={{ height: 0, opacity: 0 }}
            transition={{ duration: 0.3, ease: 'easeInOut' }}
          >
            <div className="p-4 border-t border-gray-200">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <p><strong>Connector ID:</strong> {data_info.connector_id}</p>
                  <p><strong>Description:</strong> {data_info.description}</p>
                  <p><strong>ID:</strong> {data_info.id}</p>
                  <p><strong>Metadata:</strong> {JSON.stringify(data_info.metadata)}</p>
                  <p><strong>Path:</strong> {data_info.path}</p>
                  <p><strong>Tags:</strong> {data_info.tags.join(', ')}</p>
                </div>
                <div>
                  <p><strong><u>Schema:</u></strong></p>
                  <ul className="list-disc pl-6">
                    {Object.entries(data_info.schema).map(([key, value]) => (
                      <li key={key}><strong>{key}:</strong> {value}</li>
                    ))}
                  </ul>
                </div>
              </div>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
};

export default CustomAccordion;
