// app/api/auth.js
import { NextResponse } from 'next/server';

export async function GET(request) {
  const sid = await generateSessionId();

  // Set the 'sid' cookie in the response headers
  const response = NextResponse.json({ message: 'Authentication successful' });
  response.cookies.set('sid', sid, { path: '/' });

  return response;
}

async function generateSessionId() {
  const apiEndpoint = 'http://localhost:3001/anonymouslogin';

  try {
    const response = await fetch(apiEndpoint, {
      method: 'POST',
      cache: 'no-store',
      headers: {
        'Content-Type': 'application/json',
      },
    });


    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    const data = await response.json();

    if (data.token) {
      return data.token;
    } else {
      throw new Error('Token not found in the API response');
    }
  } catch (error) {
    console.error('Error fetching token:', error);
    throw error;
  }
}
