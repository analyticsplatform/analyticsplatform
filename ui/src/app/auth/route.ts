// auth/route.ts

import { NextResponse } from 'next/server';
import { v4 as uuidv4 } from 'uuid';
import { getDb } from '../lib/db';

const db = getDb();

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const sessionId = searchParams.get('sessionId');

  if (!sessionId) {
    return NextResponse.json({ error: 'sessionId is required' }, { status: 400 });
  }

  try {
    const sessionData = await db.getSession(sessionId);

    if (!sessionData) {
      return NextResponse.json({ error: 'Session not found' }, { status: 401 });
    }

    return NextResponse.json({ message: 'Session is valid' }, { status: 200 });
  } catch (error) {
    console.error('Error retrieving session:', error);
    return NextResponse.json({ error: 'Internal server error' }, { status: 500 });
  }
}

export async function POST(request: Request) {
  try {
    const { clientIpAddress } = await request.json();

    const sessionId = uuidv4();
    await db.setSession(sessionId, { clientIpAddress });

    return NextResponse.json({ sessionId }, { status: 200 });
  } catch (error) {
    console.error('Error creating session:', error);
    return NextResponse.json({ error: 'Internal server error' }, { status: 500 });
  }
}
