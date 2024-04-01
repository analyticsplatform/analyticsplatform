// app/SetSidCookie.tsx
'use client';

import { useEffect } from 'react';

export default function SetSidCookie() {
  useEffect(() => {
    let isMounted = true;

    const fetchSidCookie = async () => {
      const sidCookie = document.cookie.includes('sid=');

      if (!sidCookie) {
        try {
          console.log("auth: calling endpoint");
          await fetch('/auth');
        } catch (error) {
          console.error('Failed to fetch sid cookie:', error);
        }
      }

      console.log("auth: session exists");
    };

    fetchSidCookie();

    return () => {
      isMounted = false;
    };
  }, []);

  return null;
}
