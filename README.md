# Gimme token

Get Bearer token from aws amplify but conveniently!

### Usage

Gimme token using `default` profile:
```bash
gimme-token
```

Gimme token from `my_custom` profile:
```bash
gimme-token --profile my_custom
```

Gimme token and copy to clipboard example:
```bash
gimme-token | wl-copy
```

### Install
- Download latest binary build
- Place in `~/.local/bin` (Ensure this is in your path)

### Config
Running `gimmie-token` for the first time will create an empty config file in your config directory. (Usually `~/.config/gimme-token/config.toml`)

Config example:
```toml
# By default the `default` profile will be used.
# Here you are able to override this.
default_profile = "default"

# The default profile is used when no arguments are provided,
# or as a base for other profiles.
[profile.default]
username = ""
password = ""

awsRegion = ""
awsUserPool = ""
awsClientId = ""

# Define a custom profile which can be referenced with `--profile <name>`
[profile.my_profile]
username = ""
password = ""

awsRegion = ""
awsUserPool = ""
awsClientId = ""

# You can define a partial profile as long as the remaining
# fields will be covered by the `default` profile.
[profile.my_partial]
username = ""
```

### Source

To install dependencies:

```bash
bun install
```

To run:

```bash
bun run start
```

To compile binary:

```bash
bun run build
```
