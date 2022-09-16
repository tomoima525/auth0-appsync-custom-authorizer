import { Stack, StackProps } from "aws-cdk-lib";
import { Construct } from "constructs";
import { AppSyncSetup } from "./appsync-stack";

export class DevelopmentTemplateStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    new AppSyncSetup(this, "appsync", {});
  }
}
