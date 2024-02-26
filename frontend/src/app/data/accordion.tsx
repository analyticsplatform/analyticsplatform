"use client"

import { Accordion } from 'flowbite-react'; 

const DataAccordion = () => {
  return (
    <Accordion collapseAll className="w-11/12 mb-4">
      <Accordion.Panel>
        <Accordion.Title>Dataset Name</Accordion.Title>
        <Accordion.Content>
          <p>Dataset information goes here</p>
        </Accordion.Content>
      </Accordion.Panel>
    </Accordion>
  )
}

export default DataAccordion;
