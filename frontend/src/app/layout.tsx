import Link from 'next/link';
import { Inter } from 'next/font/google';
import SidebarNav from './menu'
import { cookies } from 'next/headers';
import AuthHandler from './components/auth.ts';

import "./globals.css";

// Load the "Inter" font with the "latin" subset specified
const inter = Inter({
  weight: '400',
  subsets: ['latin']
});


export default function RootLayout({ children }) {
  return (
    <html lang="en">
      <head>
        {/* Head content */}
        <title>Analytics Platform</title>
      </head>
      <body>
        <div className="flex flex-col md:flex-row min-h-screen bg-sky-50">
          <SidebarNav className="w-64 min-h-screen bg-apblue" />
          <div className="flex-1 sb:ml-64">
            {children}
          </div>
        </div>
        <AuthHandler />
      </body>
    </html>
  );
}
