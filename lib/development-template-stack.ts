import { Stack, StackProps } from "aws-cdk-lib";
import { Construct } from "constructs";

export class DevelopmentTemplateStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);
    // Reference your resouces
    // See how to reference external resources https://docs.aws.amazon.com/cdk/v2/guide/resources.html
    // const s3 = s3.Bucket.fromBucketArn(this, 'MyBucket', 'arn:aws:s3:::my-bucket-name');
  }
}
