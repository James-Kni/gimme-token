import { TOML } from 'bun';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'fs';
import { dirname, join } from "path";
import z, { ZodError } from 'zod';

const GimmeConfigSchema = z.object({
  user: z.object({
    username: z.string(),
    password: z.string(),
  }),
  aws: z.object({
    region: z.string().default("eu-west-2"),
    userPool: z.string(),
    clientId: z.string(),
  }),
});

export type GimmeConfig = z.infer<typeof GimmeConfigSchema>;

const getConfigPath = () => {
  const xdgConfig = process.env.XDG_CONFIG_HOME;
  const home = process.env.HOME;

  if (xdgConfig) {
    return join(xdgConfig, "gimme-token", "config.toml");
  } else if (home) {
    return join(home, ".config", "gimme-token", "config.toml");
  } else {
    throw new Error("Unable to find config location");
  }
}

const ensureConfigExists = (path: string) => {
  if (!existsSync(path)) {
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, "", "utf8");
    console.log(`Empty config file created at ${path}`);
    console.log("You should probably edit this.");
    process.exit(1);
  }
};

export const loadConfig = (): GimmeConfig => {
  const path = getConfigPath();

  ensureConfigExists(path);

  const content = readFileSync(path, "utf8");
  const configData = TOML.parse(content);

  try {
    const validatedConfig = GimmeConfigSchema.parse(configData);
    return validatedConfig;
  } catch (error) {
    console.log(z.prettifyError(error as ZodError))
    process.exit(1);
  }

}
