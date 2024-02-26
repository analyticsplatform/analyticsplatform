import Link from 'next/link';
import { Inter } from 'next/font/google';
import SidebarNav from './menu'

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
      </head>
      <body>
        <div className="flex flex-col md:flex-row min-h-screen">
          <SidebarNav className="w-64 min-h-screen bg-apblue" />
          <div className={`flex-1`}>
            {children}
          </div>
        </div>
      </body>
    </html>
  );
}
