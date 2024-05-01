// app/layout.tsx

import { Inter } from 'next/font/google';
import { cookies } from 'next/headers';
import { Suspense, ReactNode } from 'react';
import SidebarNav from './menu';
import "./globals.css";

const inter = Inter({
  weight: '400',
  subsets: ['latin']
});

type RootLayoutProps = {
  children: ReactNode;
};

export default function RootLayout({ children }: RootLayoutProps) {
  return (
    <html lang="en">
      <head>
        <title>Analytics Platform</title>
      </head>
      <body className={inter.className}>
        <div className="flex flex-col md:flex-row min-h-screen bg-gray-100">
          <Suspense fallback={<LoadingPage />}>
            <SidebarNav />
            <div className="flex-1 sb:ml-64">{children}</div>
          </Suspense>
        </div>
      </body>
    </html>
  );
}

const LoadingPage = () => {
  return (
    <div className="flex-1 flex items-center justify-center">
      <div className="flex items-center justify-center w-full">
        <div className="w-16 h-16 border-4 border-gray-300 border-t-gray-600 rounded-full animate-spin"></div>
      </div>
    </div>
  );
};
