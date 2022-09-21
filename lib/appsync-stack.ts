import {
  aws_iam as iam,
  aws_lambda_nodejs as lambda_nodejs,
  aws_lambda as lambda,
  CfnOutput,
  Expiration,
  Duration,
} from "aws-cdk-lib";
import * as appsync from "@aws-cdk/aws-appsync-alpha";
import { Construct } from "constructs";
import * as path from "path";
import { RetentionDays } from "aws-cdk-lib/aws-logs";
import { eventGetResolver } from "./resolvers/eventGet";
import { AUDIENCE, JWKS_URI, TOKEN_ISSUER } from "./config";

interface AppSyncSetupProps {}
console.log("==== ", { TOKEN_ISSUER, AUDIENCE, JWKS_URI });
export class AppSyncSetup extends Construct {
  constructor(scope: Construct, id: string, props: AppSyncSetupProps) {
    super(scope, id);

    // cloud watch role
    const cloudwatchRole = new iam.Role(this, "ApiCloudWatchRole", {
      assumedBy: new iam.ServicePrincipal("appsync.amazonaws.com"),
      managedPolicies: [
        iam.ManagedPolicy.fromAwsManagedPolicyName(
          "service-role/AWSAppSyncPushToCloudWatchLogs",
        ),
      ],
    });

    // authorizer
    const authorizerLambda = new lambda.Function(this, "test function", {
      description: "Testing authorizer input",
      code: lambda.Code.fromAsset("target/lambda/authorizer/bootstrap.zip"),
      runtime: lambda.Runtime.PROVIDED_AL2,
      handler: "bootstrap",
      environment: {
        RUST_BACKTRACE: "1",
        AUDIENCE,
        TOKEN_ISSUER,
        JWKS_URI,
      },
      logRetention: RetentionDays.ONE_WEEK,
    });
    // authorizer
    // const authorizerLambda = new lambda_nodejs.NodejsFunction(
    //   this,
    //   "AuthorizerHandler",
    //   {
    //     runtime: lambda.Runtime.NODEJS_16_X,
    //     memorySize: 128,
    //     handler: "handler",
    //     entry: path.join(
    //       `${__dirname}/../`,
    //       "functions",
    //       "authorizer/index.ts",
    //     ),
    //     environment: {
    //       AUDIENCE,
    //       TOKEN_ISSUER,
    //       JWKS_URI,
    //     },
    //     logRetention: RetentionDays.ONE_WEEK,
    //   },
    // );

    // Creates the AppSync API
    const api = new appsync.GraphqlApi(this, "GraphqlApi", {
      name: "HelloGraphql",
      authorizationConfig: {
        defaultAuthorization: {
          authorizationType: appsync.AuthorizationType.LAMBDA,
          lambdaAuthorizerConfig: {
            handler: authorizerLambda,
            validationRegex: `^Bearer [-0-9a-zA-z\.]*$`,
          },
        },
        // for admin API
        additionalAuthorizationModes: [
          {
            authorizationType: appsync.AuthorizationType.API_KEY,
            apiKeyConfig: {
              expires: Expiration.after(Duration.days(365)),
            },
          },
        ],
      },
      schema: appsync.Schema.fromAsset(
        path.join(`${__dirname}/../`, "graphql", "schema.graphql"),
      ),
      logConfig: {
        fieldLogLevel: appsync.FieldLogLevel.ERROR,
        role: cloudwatchRole,
      },
      xrayEnabled: true,
    });

    // lambda that handles appsync request
    const appSyncHandlerLambda = new lambda_nodejs.NodejsFunction(
      this,
      "AppSyncHandler",
      {
        runtime: lambda.Runtime.NODEJS_16_X,
        memorySize: 256,
        handler: "handler",
        entry: path.join(`${__dirname}/../`, "functions", "appSync/index.ts"),
        logRetention: RetentionDays.ONE_WEEK,
      },
    );

    // DataSource: Allow AppSync invoke Lambda function
    const dataSourceIamRole = new iam.Role(this, "DataSourceIamRole", {
      assumedBy: new iam.ServicePrincipal("appsync.amazonaws.com"),
    });

    const lambdaInvokePolicy = new iam.Policy(this, "LambdaInvokePolicy", {
      policyName: "LambdaInvokePolicy",
      statements: [
        new iam.PolicyStatement({
          effect: iam.Effect.ALLOW,
          actions: ["lambda:InvokeFunction"],
          resources: [appSyncHandlerLambda.functionArn],
        }),
      ],
    });

    dataSourceIamRole.attachInlinePolicy(lambdaInvokePolicy);

    const lambdaDs = new appsync.LambdaDataSource(this, "LambdaDs", {
      api,
      lambdaFunction: appSyncHandlerLambda,
      name: "LambdaDs",
      serviceRole: dataSourceIamRole,
    });

    // Attach resolver
    eventGetResolver(lambdaDs);

    // Prints out the AppSync GraphQL endpoint to the terminal
    new CfnOutput(this, "GraphQLAPIURL", {
      value: api.graphqlUrl,
    });

    new CfnOutput(this, "GraphQLAPIKey", {
      value: api.apiKey || "",
    });
  }
}
