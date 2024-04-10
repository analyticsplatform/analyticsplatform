import { Construct } from 'constructs';
import * as cdk from 'aws-cdk-lib';
import * as dynamodb from 'aws-cdk-lib/aws-dynamodb';
import * as ec2 from 'aws-cdk-lib/aws-ec2';
import * as ecr from 'aws-cdk-lib/aws-ecr';
import * as ecs from 'aws-cdk-lib/aws-ecs';
import * as elbv2 from 'aws-cdk-lib/aws-elasticloadbalancingv2';
import * as iam from 'aws-cdk-lib/aws-iam';
import * as logs from 'aws-cdk-lib/aws-logs';

export interface InfrastructureProps {
  vpc: ec2.IVpc;
  listener: elbv2.ApplicationListener; 
}

export class Infrastructure extends Construct {
  constructor(scope: Construct, id: string, props: InfrastructureProps) {
    super(scope, id);

    // Define construct contents here
    const db = new dynamodb.TableV2(scope, 'DB', {
      partitionKey: { name: 'PK', type: dynamodb.AttributeType.STRING },
      sortKey: { name: 'SK', type: dynamodb.AttributeType.STRING },
      timeToLiveAttribute: 'expiry',

      globalSecondaryIndexes: [
        {
          indexName: 'GSI1',
          partitionKey: { name: 'GSI1PK', type: dynamodb.AttributeType.STRING },
          sortKey: { name: 'GSI1SK', type: dynamodb.AttributeType.STRING },
        },
        {
          indexName: 'GSI2',
          partitionKey: { name: 'GSI2PK', type: dynamodb.AttributeType.STRING },
          sortKey: { name: 'GSI2SK', type: dynamodb.AttributeType.STRING },
        },
      ],
    });

    const backendEcr = new ecr.Repository(scope, 'BackendRepository');
    const frontendEcr = new ecr.Repository(scope, 'FrontendRepository');

    const ecsCluster = new ecs.Cluster(scope, 'Cluster', {
      vpc: props.vpc
    });

    const backendEcsTaskRole = new iam.Role(this, 'BackendEcsTaskRole', {
      assumedBy: new iam.ServicePrincipal('ecs-tasks.amazonaws.com'),
      description: 'IAM role for Analytics Platform Backend ECS task',
    });
    db.grantReadWriteData(backendEcsTaskRole);

    const backendTaskDefinition = new ecs.FargateTaskDefinition(scope, 'BackendTaskDefinition', {
      memoryLimitMiB: 512,
      cpu: 256,
      taskRole: backendEcsTaskRole,
      runtimePlatform: {
        cpuArchitecture: ecs.CpuArchitecture.ARM64,
        operatingSystemFamily: ecs.OperatingSystemFamily.LINUX
      }
    });

    backendTaskDefinition.addContainer('BackendContainer', {
      image: ecs.ContainerImage.fromEcrRepository(backendEcr),
      essential: true,
      memoryReservationMiB: 512,
      memoryLimitMiB: 512,
      cpu: 256,
      portMappings: [{containerPort: 3001}],
      environment: {
        TABLE_NAME: db.tableName
      },
      logging: ecs.LogDrivers.awsLogs({
        streamPrefix: 'BackendService',
        logGroup: new logs.LogGroup(scope, 'AnalyticsPlatformBackendLogGroup', {
          logGroupName: 'AnalyticsPlatformBackend',
          retention: logs.RetentionDays.ONE_WEEK,
        }),
      }),
    });

    const backendSecurityGroup = new ec2.SecurityGroup(this, 'BackendSecurityGroup', {
      vpc: props.vpc,
      description: 'Analytics Platform Backend',
      allowAllOutbound: true,
    });
    backendSecurityGroup.connections.allowFrom(props.listener, ec2.Port.tcp(3001));

    const backendService = new ecs.FargateService(scope, "BackendService", {
      cluster: ecsCluster,
      taskDefinition: backendTaskDefinition,
      desiredCount: 1,
      assignPublicIp: true,
      enableECSManagedTags: true,
      enableExecuteCommand: true,
      securityGroups: [backendSecurityGroup],
      propagateTags: ecs.PropagatedTagSource.TASK_DEFINITION,
      vpcSubnets: { subnetType: ec2.SubnetType.PUBLIC }
    });

    props.listener.addTargets('BackendTargets', {
      port: 3001,
      protocol: elbv2.ApplicationProtocol.HTTP,
      targets: [backendService],
      healthCheck: { path: "/health" }
    });
  }
}
