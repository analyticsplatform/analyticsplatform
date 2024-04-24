import React from 'react';
import Accordion from './accordion';
import { cookies } from 'next/headers';


async function getDatasets() {
  const cookieStore = cookies();
  const sidCookieValue = cookieStore.get('sid')?.value;

  try {
    const res = await fetch(`${process.env.API_URL}/datasets`, {
      headers: {
        'Authorization': `Bearer ${sidCookieValue}`
      }
    });

    if (res.status === 401) {
      // Handle 401 Unauthorized error
      const errorMessage = await res.text();
      console.error(`Unauthorized: ${errorMessage}`);
      throw new Error('Unauthorized');
    }

    if (!res.ok) {
      // Handle other non-successful status codes
      throw new Error(`HTTP error! status: ${res.status}`);
    }

    const data = await res.json();
    return data;
  } catch (error) {
    console.error('Error fetching datasets:', error);
    throw error;
  }
}

const exampleDataset = {
  name: "Population",
  provider: "Department of Planets",
  connector_id: "ABC123",
  description: "How many people are there in the world?",
  id: "123ABC",
  metadata: {"requested_by": "someone"},
  path: "warehouse.data",
  tags: ["world"],
  schema: {"planet": "string", "value": "number"}
}

export default async function Data() {
  const cookieStore = cookies();
  const sidCookieValue = cookieStore.get('sid')?.value;

  try {
    const datasets = await getDatasets();

    return (
      <main className="flex flex-col min-h-screen items-start p-6 lg:p-24">
        <h1 className="w-full mb-8 text-2xl font-bold text-blue-950 md:text-4xl dark:text-white text-left">Data</h1>
        <div className="w-full">
          <Accordion data_info={datasets[0]}/>
        </div>
      </main>
    );
  } catch (error) {
    console.error('Error fetching datasets:', error);

    return (
      <main className="flex flex-col min-h-screen items-start p-6 lg:p-24">
        <h1 className="w-full mb-8 text-2xl font-bold text-blue-950 md:text-4xl dark:text-white text-left">Data</h1>
        <div className="w-full">
          <Accordion data_info={exampleDataset}/>
          <Accordion data_info={exampleDataset}/>
          <Accordion data_info={exampleDataset}/>
          <Accordion data_info={exampleDataset}/>
          <Accordion data_info={exampleDataset}/>
          <Accordion data_info={exampleDataset}/>
          <Accordion data_info={exampleDataset}/>
        </div>
      </main>
    );
  }
}
