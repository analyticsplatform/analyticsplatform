// lib/db.ts

import { DynamoDB } from '@aws-sdk/client-dynamodb';
import { marshall, unmarshall } from '@aws-sdk/util-dynamodb';

const dynamoClient = process.env.LOCAL
  ? new DynamoDB({
      region: 'localhost',
      endpoint: 'http://localhost:8090',
      credentials: {
        accessKeyId: process.env.AWS_ACCESS_KEY_ID || '',
        secretAccessKey: process.env.AWS_SECRET_ACCESS_KEY || '',
      },
    })
  : new DynamoDB({
      region: process.env.AWS_REGION || '',
      credentials: {
        accessKeyId: process.env.AWS_ACCESS_KEY_ID || '',
        secretAccessKey: process.env.AWS_SECRET_ACCESS_KEY || '',
      },
    });

const tableName = process.env.TABLE_NAME || '';

export async function getSession(sessionId: string) {
  const params = {
    TableName: tableName,
    Key: marshall({
      PK: sessionId,
    }),
  };

  try {
    const { Item } = await dynamoClient.getItem(params);
    return Item ? unmarshall(Item) : null;
  } catch (error) {
    console.error('Error retrieving record:', error);
    throw error;
  }
}

export async function setSession(sessionId: string, data: Record<string, any>) {
  const params = {
    TableName: tableName,
    Item: marshall({
      PK: sessionId,
      ...data,
    }),
  };

  try {
    await dynamoClient.putItem(params);
    console.log(`Record created with sessionId: ${sessionId}`);
  } catch (error) {
    console.error('Error creating record:', error);
    throw error;
  }
}
