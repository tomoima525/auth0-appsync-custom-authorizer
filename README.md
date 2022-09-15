# CDK lambda development template

This is a CDK template to develop lambda functions in your isolated environment

# Why we need this?

When you are building a lambda function using CDK in a team, your functions get overriden by other developers functions when they deploy. By having isolated CDK environment which imports external resources (e.g. DynamoDB, S3), we can safely develop our lambda functions. Once you confirmed that your function is working properly in your environment, you can copy your code to your base CDK project.

<p align="center">
<img src="https://user-images.githubusercontent.com/6277118/179318780-e5110421-f945-40fa-acdc-514b3945d32c.png" width=800px />
</p>

# How to use

1. Set your development Stack name

- create .env file and add your setup

```
YOUR_NAME=tomo
```

- When you deploy your CDK, it will be named as `DevStack${yourname}`

2. Add your external resources & lambda function

```
export class DevelopmentTemplateStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);
    // Reference your resouces
    // See how to reference external resources https://docs.aws.amazon.com/cdk/v2/guide/resources.html

    const s3Bucket = s3.Bucket.fromBucketArn(this, 'MyBucket', 'arn:aws:s3:::my-bucket-name');

    // Set your lambda
    cont yourLambda = new lambda_nodejs.NodejsFunction(this, "yourlambda", {
      runtime: lambda.Runtime.NODEJS_16_X,
      handler: "handler",
      entry: path.join(`${__dirname}/../`, "functions", "yourlambda/index.ts"),
      environment: {
        BUCKET: props.s3Bucket.bucketName,
      },
    });

    props.s3Bucket.grantReadWrite(yourLambda);

  }
}
```

3. Your lambda

- This project uses yarn 2+ workspace.
- Add your function under `functions` (e.g. `functions/yourlambda/index.ts`)
- you need to run `yarn install` at the root of the project so that yarn can recognize your function.
- If you want add other dependencies, call `yarn init` under `functions/yourlambda` then `yarn add {dependecies you want to add}`

4. Deploy

```
yarn cdk:deploy
```

# Tips

### Accessing lambda layer

You can use submodules to access the lambda layer

```
git submodule add git@github.com:yourproject/main-cdk.git main-cdk
```

Then add the path in `tsconfig.json`

```
{
  ...
      "paths": {
      "/opt/nodejs/s3": ["main-cdk/functions/layers/awsservice/nodejs/s3"]
    }
}
```

and when you update submodule

```
git submodule update --recursive --remote --merge
```

