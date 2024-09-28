# Shelf

## Description:

Shelf is a free and open source self hosted artifact registry that supports multiple package types:

Supported Packages:

- [x] Cargo
- [ ] NPM -> planned
- [ ] Nuget -> planned

## Configuration

| Name         | Description                                                                        | Default                                         |
|--------------|------------------------------------------------------------------------------------|-------------------------------------------------|
| STORAGE_TYPE | Describes the type of storage where you want to store you're artifacts [LOCAL, S3] | LOCAL                                           |
| DATABASE_URL | The url to the postgres database                                                   | postgres://postgres:password@localhost/postgres |
| RUST_LOG     | The log level you want to use                                                      | info                                            |
| BASE_URL     | The url on which this application is reachable                                     | http://localhost:6300                           |
