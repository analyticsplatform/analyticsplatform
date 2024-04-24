// lib/db.ts

'use server'

import { DynamoDB } from '@aws-sdk/client-dynamodb';
import { marshall, unmarshall } from '@aws-sdk/util-dynamodb';

class DynamoDBClient {
  private client: DynamoDB;
  private tableName: string;

  constructor(client: DynamoDB, tableName: string) {
    this.client = client;
    this.tableName = tableName;
  }

  async getSession(sessionId: string) {
    const params = {
      TableName: this.tableName,
      Key: marshall({
        sessionid: sessionId,
      }),
    };

    try {
      const { Item } = await this.client.getItem(params);
      return Item ? unmarshall(Item) : null;
    } catch (error) {
      console.error('Error retrieving record:', error);
      throw error;
    }
  }

  async setSession(sessionId: string, data: Record<string, any>) {
    const params = {
      TableName: this.tableName,
      Item: marshall({
        sessionid: sessionId,
        ...data,
      }),
    };

    try {
      await this.client.putItem(params);
      console.log(`Record created with sessionId: ${sessionId}`);
    } catch (error) {
      console.error('Error creating record:', error);
      throw error;
    }
  }
}

async function getDb() {
  const client = process.env.LOCAL
    ? new DynamoDB({
        region: 'localhost',
        endpoint: 'http://localhost:8090',
        credentials: {
          accessKeyId: process.env.AWS_ACCESS_KEY_ID || '',
          secretAccessKey: process.env.AWS_SECRET_ACCESS_KEY || '',
        },
      })
    : new DynamoDB({
      region: process.env.AWS_REGION,
      credentials: {
        accessKeyId: process.env.AWS_ACCESS_KEY_ID || '',
        secretAccessKey: process.env.AWS_SECRET_ACCESS_KEY || '',
        sessionToken: process.env.AWS_SESSION_TOKEN || '',
      }
    });

  const tableName = process.env.TABLE_NAME || '';
  return new DynamoDBClient(client, tableName);
}

export { getDb };
