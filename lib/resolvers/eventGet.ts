import * as appsync from "@aws-cdk/aws-appsync-alpha";

export const eventGetResolver = (lambdaDs: appsync.LambdaDataSource) => {
  lambdaDs.createResolver({
    fieldName: "eventGet",
    typeName: "Query",
  });
};
