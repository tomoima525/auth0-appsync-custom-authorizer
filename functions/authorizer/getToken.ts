export const getToken = (authorizationToken: string) => {
  if (!authorizationToken) {
    throw new Error('Expected "authorizationToken" parameter to be set');
  }
  const match = authorizationToken.match(/^Bearer (.*)$/);
  if (!match || match.length < 2) {
    throw new Error(
      `Invalid Authorization token - ${authorizationToken} does not match "Bearer .*"`,
    );
  }
  return match[1];
};
