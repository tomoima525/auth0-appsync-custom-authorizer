#!/usr/bin/env node
import "source-map-support/register";
import * as cdk from "aws-cdk-lib";
import { DevelopmentTemplateStack } from "../lib/development-template-stack";

const account = process.env.ACCOUNT;
console.log("==== Account", account);

const app = new cdk.App();
new DevelopmentTemplateStack(app, `Auth0CustomStack`, {
  env: {
    region: "us-west-2",
    account,
  },
});
