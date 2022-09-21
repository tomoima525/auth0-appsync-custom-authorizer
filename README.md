This repository showcases authorizing access against AppSync graphql with a custom lambda authorizer using Auth0.

- Auth0 provides an access token as JSON web token
- lambda authorizer verifies access token on the request header
- If verified, AppSync resolves access to AWS resources

## Preparation

- Create Auth0 API for authorization. See [this link](https://auth0.com/docs/customize/integrations/aws/aws-api-gateway-custom-authorizers#create-an-auth0-api)

## Usage

Set up `.env` file

```
ACCOUNT={your aws account}
JWKS_URI=https://xxx.auth0.com/.well-known/jwks.json
AUDIENCE=https://xxxxxx.appsync-api.us-west-2.amazonaws.com // The one you set at Preparation
TOKEN_ISSUER=https://xxxx.auth0.com/ // your Auth0 domain
```

```
yarn install
yarn bootstrap // required only once
yarn cdk:deploy
```

## Testing

Generate token. See [this link](https://auth0.com/docs/secure/tokens/access-tokens/get-access-tokens)

```
curl --request POST \
  --url 'https://YOUR_DOMAIN/oauth/token' \
  --header 'content-type: application/x-www-form-urlencoded' \
  --data grant_type=client_credentials \
  --data client_id=YOUR_CLIENT_ID \
  --data client_secret=YOUR_CLIENT_SECRET \
  --data audience=YOUR_API_IDENTIFIER
```

`YOUR_API_IDENTIFIER` will be the one you set at Preparation section

# Write Lambda in Rust

### Setup your environment

Install [cargo-lambda](https://github.com/cargo-lambda/cargo-lambda)

### Adding a new project

- Add project path `members` under `<root>/Cargo.toml`

```
members = [
  "fn/authorizer"
  "fn/your-function"
]
```

- Call `cargo lambda new your-function` under `fn/` directory

# References

https://aws.amazon.com/blogs/mobile/appsync-lambda-auth/
https://auth0.com/docs/customize/integrations/aws/aws-api-gateway-custom-authorizers
