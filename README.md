# Smart Config Manager

A Rust-based CLI to **merge** environment-specific configurations with a base config and **compare** final configs across environments. Ideal for teams wanting a single source of truth for app settings, quick environment diffs, and a self-contained binary with minimal dependencies.

## Features

- **Dynamic Merging**: Combine `base.yaml` with an environment-specific YAML (`dev.yaml`, `staging.yaml`, etc.)—no typed structs required.
- **Line-by-Line Diffs**: Compare two merged environments (`dev` vs `staging`, etc.) in a human-readable format.
- **Flexible Config Paths**: Store your YAML configs anywhere and specify the directory via `--config-path`.
- **Single Binary**: Compiles to a single executable (no extra runtime needed).
- **Extensible**: Add placeholders, schema validation, or any other custom logic as needed.

## Table of Contents

- [Requirements](#requirements)
- [Installation](#installation)
  - [Build from Source](#build-from-source)
  - [Via Cargo](#via-cargo)
- [Configuration Structure](#configuration-structure)
- [Usage](#usage)
  - [Subcommands](#subcommands)
  - [Examples](#examples)
- [Roadmap / Future Features](#roadmap--future-features)
- [Contributing](#contributing)
- [License](#license)

---

## Requirements

- **Rust** 1.60+ (if building from source)
- **Cargo** (if installing via `cargo install`)
- Linux, macOS, or Windows (64-bit recommended)

_(If you distribute prebuilt binaries, end users won’t need Rust/Cargo.)_

---

## Installation

### Build from Source

1. **Clone** the repository:
   ```bash
   git clone https://github.com/your-username/smart-config-manager.git
   cd smart-config-manager
   ```
2. **Build** the release binary:
   ```bash
   cargo build --release
   ```
3. **Run** the tool (on Unix-like systems):
   ```bash
   ./target/release/smartconfig --help
   ```

### Via Cargo

If you publish this tool to [crates.io](https://crates.io/):

```bash
cargo install smart-config-manager
```

Then run:

```bash
smart-config --help
```

_(Adjust the crate name if you publish it under something else.)_

---

## Configuration Structure

By default, the CLI looks in a directory called `config/` for your YAML files:

```
smart-config-manager/
├─ config/
│  ├─ base.yaml
│  ├─ dev.yaml
│  ├─ staging.yaml
│  └─ prod.yaml
├─ ...
```

- **base.yaml**: Common settings shared across all environments.
- **<env>.yaml**: Override or extend `base.yaml` for each environment (`dev`, `staging`, `prod`, etc.).
- You can **specify a custom path** using `--config-path /some/other/path`.

Example **base.yaml**:

```yaml
logging:
  level: info
  format: text

service:
  port: 8080
  name: MyApp
```

Example **dev.yaml**:

```yaml
logging:
  level: debug

database:
  host: localhost
  user: dev_user
  password: dev_secret
```

---

## Usage

Run `smartconfig --help` (or `./smartconfig` if you built locally) to see all commands and options:

```
smartconfig 0.1.0
A CLI to manage environment configs

USAGE:
    smartconfig <COMMAND>

COMMANDS:
    build    Merge base + environment override
    diff     Compare final config of two environments
    help     Prints this message or the help of the given subcommand(s)
```

### Subcommands

1. **build**

   - Merges `base.yaml` + `<env>.yaml` into a single merged YAML.
   - Can output to stdout or write to a file.
   - Supports a custom config directory path.

2. **diff**
   - Generates a line-by-line diff of two fully merged configs.
   - Great for seeing what changed between `dev` and `staging`, etc.

### Examples

1. **Build** for `dev` environment, print to stdout:

   ```bash
   smartconfig build --env dev
   ```

   This will look for `config/base.yaml` + `config/dev.yaml` and print the merged YAML to stdout.

2. **Build** for `staging` and output to file:

   ```bash
   smartconfig build --env staging --output staging.final.yaml
   ```

3. **Diff** `dev` vs `staging`:

   ```bash
   smartconfig diff --env1 dev --env2 staging
   ```

   Displays color-coded differences between the final dev and staging configs.

4. **Specify a custom config path**:

   ```bash
   smartconfig build --env prod --config-path ./my_configs
   ```

   Looks for `./my_configs/base.yaml` and `./my_configs/prod.yaml`.

---

## Roadmap / Future Features

- **Placeholder Replacement**: e.g., `{{env:MY_SECRET}}` -> `$MY_SECRET` at runtime.
- **Schema Validation**: Validate merged configs with JSON Schema to catch missing or invalid fields.
- **Structured Diffs**: Compare YAML trees to highlight changed keys/values rather than raw text lines.
- **Plugin Architecture**: Allow custom merge strategies or transformations.

---

## Contributing

1. **Fork** the repo and clone your fork locally.
2. **Create a branch** for your feature or fix: `git checkout -b feature/my-improvement`.
3. **Commit and push**:
   ```bash
   git commit -m "Add new feature"
   git push origin feature/my-improvement
   ```
4. **Open a Pull Request** in the original repo, describing your changes.

Please also check out the [issues](https://github.com/your-username/smart-config-manager/issues) for open tasks or bugs. We welcome contributions of all kinds, from code to documentation improvements!

---

## License

This project is licensed under the [MIT License](LICENSE). You’re free to use, modify, and distribute it as allowed under the license terms.

---

**Happy Configuring!**  
If you have questions or suggestions, feel free to file an [issue](https://github.com/your-username/smart-config-manager/issues) or open a PR. We’d love to hear your feedback and contributions!
