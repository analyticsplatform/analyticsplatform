// app/layout.tsx
import { Inter } from 'next/font/google';
import { cookies } from 'next/headers';
import { Suspense } from 'react';
import "./globals.css";
import dynamic from 'next/dynamic';

const inter = Inter({
  weight: '400',
  subsets: ['latin']
});

const ProfileComponent = dynamic(() => import('./profile.tsx').then((mod) => mod.Profile), {
  loading: () => <LoadingPage />,
  ssr: true,
});

export default function RootLayout({ children }) {
  return (
    <html lang="en">
      <head>
        <title>Analytics Platform</title>
      </head>
      <body className={inter.className}>
        <div className="flex flex-col md:flex-row min-h-screen bg-sky-50">
          <Suspense fallback={<LoadingPage />}>
            <ProfileComponent>{children}</ProfileComponent>
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
