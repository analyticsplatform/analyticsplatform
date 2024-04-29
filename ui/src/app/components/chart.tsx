// components/Chart.tsx

'use client'

import React, { useRef, useEffect } from 'react';
import {
  Chart as ChartJS,
  LineElement,
  BarElement,
  PointElement,
  LinearScale,
  Title,
  CategoryScale,
  Legend,
  Tooltip,
  LineController,
  BarController,
} from 'chart.js';

ChartJS.register(
  LineElement,
  BarElement,
  PointElement,
  LinearScale,
  Title,
  CategoryScale,
  Legend,
  Tooltip,
  LineController,
  BarController
);

export interface ChartProps {
  data: {
    labels: string[];
    datasets: {
      label: string;
      data: number[];
      borderColor: string;
      backgroundColor: string;
    }[];
  };
  type: 'line' | 'bar' | 'pie' | 'doughnut' | 'radar' | 'polarArea' | 'bubble' | 'scatter';
}

const Chart: React.FC<ChartProps> = ({ data, type }) => {
  const chartRef = useRef<HTMLCanvasElement | null>(null);

  useEffect(() => {
    if (chartRef.current) {
      const ctx = chartRef.current.getContext('2d');
      if (ctx) {
        new ChartJS(ctx, {
          type,
          data,
          options: {
            responsive: true,
            maintainAspectRatio: false,
            scales: {
              y: {
                beginAtZero: true,
                grid: {
                  color: 'rgba(0, 0, 0, 0.05)',
                },
                ticks: {
                  color: 'rgba(0, 0, 0, 0.6)',
                },
              },
              x: {
                grid: {
                  color: 'rgba(0, 0, 0, 0.05)',
                },
                ticks: {
                  color: 'rgba(0, 0, 0, 0.6)',
                },
              },
            },
            plugins: {
              legend: {
                labels: {
                  color: 'rgba(0, 0, 0, 0.6)',
                },
              },
            },
          },
        });
      }
    }
  }, [data, type]);

  return (
    <div className="bg-white shadow-lg rounded-lg p-6 h-full">
      <canvas ref={chartRef} />
    </div>
  );
};

export default Chart;
