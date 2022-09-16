import { AppSyncAuthorizerEvent } from "aws-lambda";
import * as jwt from "jsonwebtoken";
import { JwksClient } from "jwks-rsa";
import { getToken } from "./getToken";
import { promisify } from "util";

const client = new JwksClient({
  cache: true,
  rateLimit: true,
  jwksRequestsPerMinute: 10, // Default value
  jwksUri: process.env.JWKS_URI as string,
});

const jwtOptions = {
  audience: process.env.AUDIENCE,
  issuer: process.env.TOKEN_ISSUER,
};
export const handler = async (event: AppSyncAuthorizerEvent) => {
  console.log(`event >`, JSON.stringify(event, null, 2));
  try {
    const token = getToken(event.authorizationToken);
    console.log("===== token", token);
    const decoded = jwt.decode(token, { complete: true });
    console.log("===== decoded", decoded);
    if (!decoded || !decoded.header || !decoded.header.kid) {
      throw new Error("invalid token");
    }

    const getSigningKey = promisify(client.getSigningKey);
    const key = await getSigningKey(decoded.header.kid);
    const pubKey = key?.getPublicKey() as string;
    const decodedToken = jwt.verify(token, pubKey, jwtOptions);
    console.log("===== decoded Token", decodedToken);
    const response = {
      isAuthorized: true,
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
  } catch (error) {
    console.log({ error });
    return {
      isAuthorized: false,
    };
  }
};
