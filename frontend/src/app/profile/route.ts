import { NextResponse } from 'next/server';

export async function GET(request) {
  const response = NextResponse.json({ message: 'profile info placeholder' });

  await new Promise(r => setTimeout(r, 3000));

  return response;
}
