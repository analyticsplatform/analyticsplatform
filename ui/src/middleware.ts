// middleware.ts

import { NextRequest, NextResponse } from 'next/server';

export async function middleware(request: NextRequest) {
  const sidCookie = request.cookies.get('sid');

  if (!sidCookie || !sidCookie.value) {
    // sid cookie missing or invalid, create a new session
    const clientIpAddress = request.headers.get('x-forwarded-for') || request.ip;
    const response = await fetch('http://localhost:3000/auth', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ clientIpAddress }),
    });

    if (response.ok) {
      const data = await response.json();
      const sessionId = data.sessionId;
      const cookieOptions = {
        httpOnly: true,
        secure: process.env.NODE_ENV === 'production',
        maxAge: 60 * 60 * 24 * 7, // 1 week
        path: '/',
      };
      const newResponse = NextResponse.next();
      newResponse.cookies.set('sid', sessionId, cookieOptions);
      return newResponse;
    } else {
      // Handle the error appropriately (e.g., return an error response)
      return NextResponse.error();
    }
  } else {
    // sid cookie exists, retrieve the session
    const response = await fetch(`http://localhost:3000/auth?sessionId=${sidCookie.value}`, {
      method: 'GET',
    });

    if (response.ok) {
      // Session is valid, continue to the intended route
      return NextResponse.next();
    } else if (response.status === 401) {
      // Session is invalid, create a new session
      const clientIpAddress = request.headers.get('x-forwarded-for') || request.ip;
      const createResponse = await fetch('http://localhost:3000/auth', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ clientIpAddress }),
      });

      if (createResponse.ok) {
        const data = await createResponse.json();
        const sessionId = data.sessionId;
        const cookieOptions = {
          httpOnly: true,
          secure: process.env.NODE_ENV === 'production',
          maxAge: 60 * 60 * 24 * 7, // 1 week
          path: '/',
        };
        const newResponse = NextResponse.next();
        newResponse.cookies.set('sid', sessionId, cookieOptions);
        return newResponse;
      } else {
        // Handle the error appropriately (e.g., return an error response)
        return NextResponse.error();
      }
    } else {
      // Handle other error statuses appropriately
      return NextResponse.error();
    }
  }
}

export const config = {
  matcher: ['/', '/data', '/map'],
};
