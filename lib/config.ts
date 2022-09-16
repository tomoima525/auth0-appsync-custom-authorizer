import * as dotenv from "dotenv"; // see https://github.com/motdotla/dotenv#how-do-i-use-dotenv-with-import
dotenv.config();
export const TOKEN_ISSUER = process.env.TOKEN_ISSUER as string;
export const AUDIENCE = process.env.AUDIENCE as string;
export const JWKS_URI = process.env.JWKS_URI as string;
