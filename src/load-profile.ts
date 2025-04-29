import { TOML } from "bun";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "fs";
import { dirname, join } from "path";
import z, { ZodError } from "zod";
import { tryCatch } from "./utils/try-catch";

const ProfileSchema = z.object({
  username: z.string(),
  password: z.string(),
  awsRegion: z.string(),
  awsUserPool: z.string(),
  awsClientId: z.string(),
});

const GimmeConfigSchema = z.object({
  default_profile: z.string().default("default"),
  profile: z.record(z.string(), ProfileSchema.partial()),
});

type GimmeConfig = z.infer<typeof GimmeConfigSchema>;
type Profile = z.infer<typeof ProfileSchema>;

const getConfigPath = () => {
  const xdgConfig = process.env.XDG_CONFIG_HOME;
  const home = process.env.HOME;

  if (xdgConfig) {
    return join(xdgConfig, "gimme-token", "config.toml");
  } else if (home) {
    return join(home, ".config", "gimme-token", "config.toml");
  } else {
    console.error("[ERROR] Unable to find config location");
    process.exit(0);
  }
};

const ensureConfigExists = (path: string) => {
  if (existsSync(path)) {
    return;
  }

  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, "", "utf8");
  console.log(`Empty config file created at ${path}`);
  console.log("You should probably edit this.");
  process.exit(0);
};

export const loadProfile = async (profileKey?: string): Promise<Profile> => {
  const path = getConfigPath();

  ensureConfigExists(path);

  const content = readFileSync(path, "utf8");
  const configData = TOML.parse(content);

  const { data: config, error: configError } = await tryCatch<
    GimmeConfig,
    ZodError
  >(GimmeConfigSchema.parseAsync(configData));

  if (configError) {
    console.error(z.prettifyError(configError as ZodError));
    process.exit(0);
  }

  if ( profileKey && !config.profile[profileKey]) {
    console.log(`Profile '${profileKey}' is not defined`);
    Object.keys(config.profile).forEach((profile) => {
      console.log(`- ${profile}`);
    });
    process.exit(0);
  }

  const mergedProfile = {
    ...config.profile[config.default_profile],
    ...(profileKey ? config.profile[profileKey] : []),
  };

  const { data: profile, error: profileError } = await tryCatch<
    Profile,
    ZodError
  >(ProfileSchema.parseAsync(mergedProfile));

  if (profileError) {
    console.error(z.prettifyError(profileError));
    process.exit(0);
  }

  return profile;
};
