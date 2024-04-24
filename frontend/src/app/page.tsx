// app/dashboard/page.tsX

import React, { useState } from 'react';
import dynamic from 'next/dynamic';
import FilterBar from './components/filter-bar'
import Chart, { ChartProps } from './components/chart'
import Feed from './components/feed'

// chart-data.ts
const chart_data: Record<string, ChartProps> = {
  lineChart: {
    data: {
      labels: ['January', 'February', 'March', 'April', 'May', 'June', 'July'],
      datasets: [
        {
          label: 'Sales',
          data: [12, 19, 3, 5, 2, 3, 20],
          borderColor: 'rgba(75, 192, 192, 1)',
          backgroundColor: 'rgba(75, 192, 192, 0.2)',
        },
        {
          label: 'Revenue',
          data: [5, 10, 15, 8, 12, 18, 25],
          borderColor: 'rgba(255, 99, 132, 1)',
          backgroundColor: 'rgba(255, 99, 132, 0.2)',
        },
      ],
    },
    type: 'line',
  },
  barChart: {
    data: {
      labels: ['Apple', 'Banana', 'Orange', 'Grape', 'Mango'],
      datasets: [
        {
          label: 'Quantity',
          data: [50, 30, 45, 25, 60],
          borderColor: 'rgba(54, 162, 235, 1)',
          backgroundColor: 'rgba(54, 162, 235, 0.2)',
        },
      ],
    },
    type: 'bar',
  },
  // Add more chart configurations as needed
};


const DashboardPage: React.FC = () => {
  return (
    <div className="flex flex-col md:flex-row p-4 gap-4 md:h-screen">
      <div className="md:w-3/4 flex flex-col gap-4">
        {/* Horizontal bar with buttons and dropdowns */}
        <FilterBar />

        {/* Chart sections */}
        <div className="md:h-1/2 flex flex-col md:flex-row gap-4">
          <div className="md:w-1/2">
            <Chart {...chart_data.lineChart} />
          </div>
          <div className="md:w-1/2">
            <Chart {...chart_data.barChart} />
          </div>
        </div>
        <div className="md:h-1/2 flex flex-col md:flex-row gap-4">
          <div className="md:w-1/2">
            <Chart {...chart_data.barChart} />
          </div>
          <div className="md:w-1/2">
            <Chart {...chart_data.lineChart} />
          </div>
        </div>
      </div>
      <div className="md:w-1/4 flex flex-col gap-4">
        <Feed />
      </div>
    </div>
  );
};

export default DashboardPage;
