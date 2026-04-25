# Cubbyman

A CLI for declaratively managing Docker containers from a TOML config file.

> [!WARNING]
> Cubbyman is still in development, and may be subject to frequent and unannounced breaking changes.

## Overview

Cubbyman reads a `cubbyfile.toml` describing your containers and boots them in your local Docker daemon, similar to Docker Compose. It can also run as a webhook server that reloads containers on demand, which is convenient for triggering redeploys from CI.

## Installation

Requires a reachable Docker socket at the local default path.

```sh
cargo install --path .
```

Or build a release binary:

```sh
cargo build --release
```

## Quick start

Create a `cubbyfile.toml` in your project directory:

```toml
[[containers]]
name = "container-cubby"
image = "cubby.no/rosvik/container-cubby:main"
env = ["USERNAME=admin", "PASSWORD=hunter2"]
mounts = ["data:/data"]
ports = ["8602:8602"]
network = "cubby"

[[containers]]
name = "redis"
image = "redis:latest"
ports = ["6379:6379"]
network = "cubby"
volumes = ["redis:/data"]
secrets = ["REDIS_PASSWORD"]
```

Then apply the config:

```sh
cubbyman --apply
```

Cubbyman will pull the images, create the networks, and run the containers with the specified settings.

A more complete example can be found in [cubbyfile.example.toml](cubbyfile.example.toml).

## CLI usage

| Flag | Description |
|---|---|
| `--status` | Print Docker daemon info |
| `--apply [file]` | Pull images, create networks, and (re)run containers |
| `--destroy [file]` | Stop and remove containers listed in the config |
| `--purge [file]` | Destroy containers and delete their images |
| `--print-config [file]` | Print the resolved config (after includes and secrets) |
| `--serve <file>` | Run the webhook server on `0.0.0.0:8600`. `<file>` will be applied when triggered by the webhook. |
| `--list-images` | List all images |
| `--list-containers` | List all containers |
| `--list-networks` | List all networks |
| `--list-volumes` | List all volumes |

If `[file]` is omitted, cubbyman looks for `cubbyfile.toml` in the current directory.

## Configuration reference

### `[[containers]]`

| Field | Type | Description |
|---|---|---|
| `name` | string | Container name. Must be unique. Existing containers with this name are replaced. |
| `image` | string | Image reference, as you would pass it to `docker pull <image>`. |
| `cmd` | array of strings | Arguments passed to the container on startup. |
| `env` | array of strings | Environment variables, formatted as `"KEY=value"`. |
| `secrets` | array of strings | Secrets pulled from the local `.env` file. Format: `"DOTENV_KEY:CONTAINER_KEY"`, or just `"KEY"` if both names match. |
| `ports` | array of strings | Port bindings, formatted as `"host:container"`. |
| `network` | string | `bridge`, `host`, `none`, `container:<name\|id>`, or any other value to create a custom bridge network with that name. |
| `mounts` | array of strings | Bind mounts, formatted as `"host_path:container_path"`. Host paths are resolved relative to the directory of the config file. |
| `volumes` | array of strings | Docker volumes, formatted as `"volume_name:container_path"`. |
| `user` | string | User to run the container as, formatted as `"user:group"`. |

### `[[logins]]`

Registry credentials used when pulling images.

| Field | Type | Description |
|---|---|---|
| `registry` | string | Registry hostname. |
| `username` | string | Registry username. |
| `password` | string | Registry password. |

### `include`

A list of other config files to include, resolved relative to the current file. Containers and logins from included files are merged into the parent.

```toml
include = ["services/web.toml", "services/db.toml"]
```

### Secrets and `.env`

Cubbyman loads `.env` files from the directory of the config file and makes them available as environment variables to the containers if they are listed in the `secrets` field.

For example, in a config file at `example/cubbyfile.toml`:

```toml
[[containers]]
name = "api"
image = "example/api:main"
env = ["PORT=8080"]
secrets = ["API_DB_PASSWORD:PASSWORD", "API_TOKEN"]
```

With this `.env` file at `example/.env`:

```env
API_DB_PASSWORD=hunter2
API_TOKEN=abc123
```

The container receives the environment variables `PASSWORD=hunter2` and `API_TOKEN=abc123`.

## Webhook server

Run cubbyman as a HTTP server that reloads containers when triggered. Useful for CI-driven redeploys.

```sh
cubbyman --serve cubbyfile.toml
```

The server listens on `0.0.0.0:8600`. To trigger a reload, call POST to `/api/cubbyman/v1/reload`, using HTTP Basic auth for authentication. Credentials are read from the `API_USERNAME` and `API_PASSWORD` environment variables (which can be supplied via a `.env` file in the working directory when running the server).

```sh
curl -u "$API_USERNAME:$API_PASSWORD" -X POST http://host:8600/api/cubbyman/v1/reload
```
