// middleware.ts

import { NextRequest, NextResponse } from 'next/server';
import { v4 as uuidv4 } from 'uuid';
import { getDb } from './app/lib/db';

async function createNewSession(request: NextRequest, response: NextResponse) {
  const newSessionId = uuidv4();
  const timestamp = Date.now();

  const ipHeader = request.headers.get('X-Forwarded-For');
  const clientIpAddress = ipHeader ? ipHeader.split(/, /)[0] : '';

  try {
    const db = await getDb();
    await db.setSession(newSessionId, { timestamp, clientIpAddress });

    response.cookies.set('sid', newSessionId, {
      httpOnly: true,
      secure: process.env.NODE_ENV === 'production',
      maxAge: 60 * 60 * 24 * 7, // 1 week
      path: '/',
    });

    console.log(`Created new session with sessionId: ${newSessionId}`);
  } catch (error) {
    console.error('Error creating session:', error);
    // Handle the error appropriately (e.g., return an error response)
  }
}

// Used to check for cookie existence and session validity.
export async function middleware(request: NextRequest) {
  const response = NextResponse.next();
  const sidCookie = request.cookies.get('sid');
  console.log(sidCookie);

  if (sidCookie && sidCookie.value) {
    const sessionId = sidCookie.value;

    try {
      const db = await getDb();
      const sessionData = await db.getSession(sessionId);

      if (!sessionData) {
        // Session not found in the database, create a new session
        await createNewSession(request, response);
      } else {
        console.log(`Session found with sessionId: ${sessionId}`);
      }
    } catch (error) {
      console.error('Error retrieving or creating session:', error);
      // Handle the error appropriately (e.g., return an error response)
    }
  } else {
    // sid cookie missing or invalid, create a new session
    await createNewSession(request, response);
  }

  return response;
}

export const config = {
  // The above middleware would only run for the "/", "/data", and "/map" paths
  matcher: ['/', '/data', '/map'],
};
