#!/usr/bin/env node
import * as dotenv from "dotenv"; // see https://github.com/motdotla/dotenv#how-do-i-use-dotenv-with-import
import { randomBytes } from "crypto";
import "source-map-support/register";
import * as cdk from "aws-cdk-lib";
import { DevelopmentTemplateStack } from "../lib/development-template-stack";

dotenv.config();
const envname = process.env.YOUR_NAME || randomBytes(10).toString("hex");
const app = new cdk.App();
new DevelopmentTemplateStack(app, `DevStack${envname}`, {
  // Rename your stack name for visibility
});
