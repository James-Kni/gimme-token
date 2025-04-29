import { Amplify, Auth } from "aws-amplify";
import { parseArgs } from "util";
import { loadProfile } from "./load-profile";
import pkjson from '../package.json';
import { tryCatch } from './utils/try-catch';

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

const profile = await loadProfile(values.profile)

Amplify.configure({
  Auth: {
    mandatorySignIn: true,
    region: profile.awsRegion,
    userPoolId: profile.awsUserPool,
    userPoolWebClientId: profile.awsClientId,
  },
});

const { error } = await tryCatch(Auth.signIn({
  username: profile.username,
  password: profile.password,
}));

if (error) {
  console.error("[ERROR] Unable to authenticate with provided profile")
  process.exit(0);
}

const token = await Auth.currentAuthenticatedUser().then(
  (u) => `Bearer ${u.signInUserSession?.idToken?.jwtToken}`,
);

process.stdout.write(token);
