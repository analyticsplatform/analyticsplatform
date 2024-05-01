// lib/db.ts

'use server'

import { DynamoDB } from '@aws-sdk/client-dynamodb';
import { marshall, unmarshall } from '@aws-sdk/util-dynamodb';
import { fromNodeProviderChain } from "@aws-sdk/credential-providers";


class DynamoDBClient {
  private client: DynamoDB;
  private tableName: string;

  constructor(client: DynamoDB, tableName: string) {
    this.client = client;
    this.tableName = tableName;
  }

  async getSession(sessionId: string) {
    const getParams = {
      TableName: this.tableName,
      Key: marshall({
        sessionid: sessionId,
      }),
    };

    try {
      const { Item } = await this.client.getItem(getParams);
      const session = Item ? unmarshall(Item) : null;

      if (session) {
        const updateParams = {
          TableName: this.tableName,
          Key: marshall({
            sessionid: sessionId,
          }),
          UpdateExpression: 'SET #requests = if_not_exists(#requests, :zero) + :incr',
          ExpressionAttributeNames: {
            '#requests': 'requests',
          },
          ExpressionAttributeValues: marshall({
            ':zero': 0,
            ':incr': 1,
          }),
        };

        await this.client.updateItem(updateParams);
      }

      return session;
    } catch (error) {
      console.error('Error retrieving record:', error);
      throw error;
    }
  }

  async setSession(sessionId: string, data: Record<string, any>) {
    const params = {
      TableName: this.tableName,
      Key: marshall({
        sessionid: sessionId,
      }),
      UpdateExpression: 'SET #data = :data, #requests = if_not_exists(#requests, :zero) + :incr',
      ExpressionAttributeNames: {
        '#data': 'data',
        '#requests': 'requests',
      },
      ExpressionAttributeValues: marshall({
        ':data': data,
        ':zero': 0,
        ':incr': 1,
      }),
    };

    try {
      await this.client.updateItem(params);
      console.log(`Record updated with sessionId: ${sessionId}`);
    } catch (error) {
      console.error('Error updating record:', error);
      throw error;
    }
  }
}

function getDb() {
  let client;
  if (process.env.IS_LOCAL) {
    client = new DynamoDB({
      region: 'localhost',
      endpoint: 'http://localhost:8090',
      credentials: {
        accessKeyId: process.env.AWS_ACCESS_KEY_ID || '',
        secretAccessKey: process.env.AWS_SECRET_ACCESS_KEY || '',
      },
    });
  } else {
    // Inside ECS container
    client = new DynamoDB({
      region: process.env.AWS_REGION,
      credentials: fromNodeProviderChain()
    });
  }

  const tableName = process.env.TABLE_NAME || '';
  return new DynamoDBClient(client, tableName);
}

export { getDb };
