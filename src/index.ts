import { Amplify, Auth } from "aws-amplify";
import { parseArgs } from "util";
import { loadConfig } from "./load-config";
import pkjson from '../package.json';

const config = loadConfig();
const defaultKey = config.default;

const { values } = parseArgs({
  args: Bun.argv,
  options: {
    profile: {
      type: "string",
      short: "p",
    },
    version: {
      type: "boolean",
      short: "V",
    },
  },
  allowPositionals: true,
});

if (values.version) {
  console.log(`Version: ${pkjson.version}`);
  process.exit(0);
}

const key = values.profile || defaultKey;
const profile = config.profile[key];

if (!profile) {
  console.log(`Profile '${key}' is not defined`);
  Object.keys(config.profile).forEach((profile) => {
    console.log(`- ${profile}`);
  });
  process.exit(0);
}

Amplify.configure({
  Auth: {
    mandatorySignIn: true,
    region: profile.aws.region,
    userPoolId: profile.aws.userPool,
    userPoolWebClientId: profile.aws.clientId,
  },
});

await Auth.signIn({
  username: profile.user.username,
  password: profile.user.password,
});

const token = await Auth.currentAuthenticatedUser().then(
  (u) => `Bearer ${u.signInUserSession?.idToken?.jwtToken}`,
);

process.stdout.write(token);
