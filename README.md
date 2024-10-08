# Shelf

## Description:

Shelf is a free and open source self hosted artifact registry that supports multiple package types:

Supported Packages:

- [x] Cargo
- [ ] NPM -> planned
- [ ] Nuget -> planned

## Configuration

| Name         | Description                                                                                                          | Default                                         |
|--------------|----------------------------------------------------------------------------------------------------------------------|-------------------------------------------------|
| STORAGE_TYPE | Describes the type of storage where you want to store you're artifacts [LOCAL, S3] currently only local is supported | LOCAL                                           |
| HTTP_BINDING | The address and port you want to bind the service on                                                                 | 0.0.0.0:6300                                    |
| DATABASE_URL | The url to the postgres database                                                                                     | postgres://postgres:password@localhost/postgres |
| RUST_LOG     | The log level you want to use                                                                                        | info                                            |
| BASE_URL     | The url on which this application is reachable                                                                       | http://localhost:6300                           |
| CONFIG_PATH  | The path to the config file                                                                                          | ./config.toml                                   |
| JWT_SECRET   | Secret for creating JWT Tokens for Service Accounts                                                                  | secret                                          |

## How to setup:

1. Provide a postgres database or use the included docker compose file to spin up a postgres database
2. Run the following command to run the migrator. The migrator will run all sql migrations:
```shell
cargo run --bin migrator
```

3. Currently, you need to use a oidc authentication server to use this software. So you need to insert a valid authority inside the config.toml file
4. To start the api run:
```shell
cargo run --bin api
```

5. To start the ui run the following commands:
```shell
# Navigate to the ui directory
cd shelf-ui

# Install packages
pnpm i

# Start the ui
pnpm dev
```