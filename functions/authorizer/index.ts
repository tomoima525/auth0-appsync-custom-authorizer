import { AppSyncAuthorizerEvent } from "aws-lambda";
export const handler = async (event: AppSyncAuthorizerEvent) => {
  console.log(`event >`, JSON.stringify(event, null, 2));
  const {
    authorizationToken,
    requestContext: { apiId, accountId },
  } = event;
  const response = {
    isAuthorized: authorizationToken === "custom-authorized",
    resolverContext: {
      userid: "test-user-id",
      info: "contextual information A",
      more_info: "contextual information B",
    },
    deniedFields: [`Mutation.createEvent`],
    ttlOverride: 10,
  };
  console.log(`response >`, JSON.stringify(response, null, 2));
  return response;
};
