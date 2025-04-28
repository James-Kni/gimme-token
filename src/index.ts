import { Amplify, Auth } from "aws-amplify";
import { loadConfig } from './load-config';

const config = loadConfig();

Amplify.configure({
  Auth: {
    mandatorySignIn: true,
    region: config.aws.region,
    userPoolId: config.aws.userPool,
    userPoolWebClientId: config.aws.clientId,
  },
});

await Auth.signIn({
  username: config.user.username,
  password: config.user.password,
});

const token = await Auth.currentAuthenticatedUser().then(
  (u) => `Bearer ${u.signInUserSession?.idToken?.jwtToken}`,
);

process.stdout.write(token);
