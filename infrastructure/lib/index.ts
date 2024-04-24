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
  baseUrl: String,
}

export class Infrastructure extends Construct {
  constructor(scope: Construct, id: string, props: InfrastructureProps) {
    super(scope, id);

    const backendDb = new dynamodb.TableV2(scope, 'DB', {
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

    const uiDb = new dynamodb.TableV2(scope, 'DBui', {
      partitionKey: { name: 'sessionid', type: dynamodb.AttributeType.STRING },

      globalSecondaryIndexes: [
        {
          indexName: 'TOKENGSI',
          partitionKey: { name: 'token', type: dynamodb.AttributeType.STRING },
        },
      ],
    });

    const backendEcr = new ecr.Repository(scope, 'BackendRepository');
    const uiEcr = new ecr.Repository(scope, 'uiRepository');

    const ecsCluster = new ecs.Cluster(scope, 'Cluster', {
      vpc: props.vpc
    });

    const backendEcsTaskRole = new iam.Role(this, 'BackendEcsTaskRole', {
      assumedBy: new iam.ServicePrincipal('ecs-tasks.amazonaws.com'),
      description: 'IAM role for Analytics Platform Backend ECS task',
    });
    backendDb.grantReadWriteData(backendEcsTaskRole);

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
        TABLE_NAME: backendDb.tableName
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
      conditions: [
        elbv2.ListenerCondition.hostHeaders([`api.${props.baseUrl}`]),
      ],
      priority: 10,
      port: 3001,
      protocol: elbv2.ApplicationProtocol.HTTP,
      targets: [backendService],
      healthCheck: { path: "/health" }
    });

    const uiEcsTaskRole = new iam.Role(this, 'UiEcsTaskRole', {
      assumedBy: new iam.ServicePrincipal('ecs-tasks.amazonaws.com'),
      description: 'IAM role for Analytics Platform UI ECS task',
    });
    uiDb.grantReadWriteData(uiEcsTaskRole);

    const uiTaskDefinition = new ecs.FargateTaskDefinition(scope, 'UiTaskDefinition', {
      memoryLimitMiB: 512,
      cpu: 256,
      taskRole: uiEcsTaskRole,
      runtimePlatform: {
        cpuArchitecture: ecs.CpuArchitecture.ARM64,
        operatingSystemFamily: ecs.OperatingSystemFamily.LINUX
      }
    });

    uiTaskDefinition.addContainer('UiContainer', {
      image: ecs.ContainerImage.fromEcrRepository(uiEcr),
      essential: true,
      memoryReservationMiB: 512,
      memoryLimitMiB: 512,
      cpu: 256,
      portMappings: [{containerPort: 3000}],
      environment: {
        TABLE_NAME: uiDb.tableName
      },
      logging: ecs.LogDrivers.awsLogs({
        streamPrefix: 'UiService',
        logGroup: new logs.LogGroup(scope, 'AnalyticsPlatformUiLogGroup', {
          logGroupName: 'AnalyticsPlatformUi',
          retention: logs.RetentionDays.ONE_WEEK,
        }),
      }),
    });

    const uiSecurityGroup = new ec2.SecurityGroup(this, 'UiSecurityGroup', {
      vpc: props.vpc,
      description: 'Analytics Platform UI',
      allowAllOutbound: true,
    });
    uiSecurityGroup.connections.allowFrom(props.listener, ec2.Port.tcp(3000));
//
    const uiService = new ecs.FargateService(scope, "UiService", {
      cluster: ecsCluster,
      taskDefinition: uiTaskDefinition,
      desiredCount: 1,
      assignPublicIp: true,
      enableECSManagedTags: true,
      enableExecuteCommand: true,
      securityGroups: [uiSecurityGroup],
      propagateTags: ecs.PropagatedTagSource.TASK_DEFINITION,
      vpcSubnets: { subnetType: ec2.SubnetType.PUBLIC }
    });

    props.listener.addTargets('UiTargets', {
      conditions: [
        elbv2.ListenerCondition.hostHeaders([props.baseUrl as string]),
      ],
      priority: 9,
      port: 3000,
      protocol: elbv2.ApplicationProtocol.HTTP,
      targets: [uiService],
      healthCheck: { path: "/" }
    });
  }
}
