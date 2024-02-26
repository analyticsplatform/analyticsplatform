"use client"
import React from 'react';
import Accordion from './accordion.tsx';

export default function Home() {

  return (
    <main className="flex min-h-screen flex-col items-center p-6 lg:p-24">
      <h1 className="mb-4 text-2xl font-bold text-gray-900 md:text-4xl dark:text-white">Data</h1>
      <Accordion />
      <Accordion />
      <Accordion />
      <Accordion />
      <Accordion />
    </main>
  );
}
