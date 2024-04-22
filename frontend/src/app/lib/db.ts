// lib/dynamodb.js
import { DynamoDB } from '@aws-sdk/client-dynamodb';
import { marshall, unmarshall } from '@aws-sdk/util-dynamodb';

const dynamoClient = new DynamoDB({ region: process.env.AWS_REGION });
const tableName = process.env.TABLE_NAME;

export async function getSession(sessionId) {
  const params = {
    TableName: tableName,
    Key: marshall({
      PK: sessionId,
    }),
  };

  try {
    const { Item } = await dynamoClient.getItem(params);
    console.log(`item: ${Item}`)
    return Item ? unmarshall(Item) : null;
  } catch (error) {
    console.error('Error retrieving record:', error);
    throw error;
  }
}
