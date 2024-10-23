<secondary-label ref="wip"/>

# Cargo

## How to use a cargo repository

When you created a cargo repository you will receive its configuration.

<img src="repo-created.png" alt="repo-created"/>

To use the repository you need to insert it into the following cargo configuration file:

```shell
    $ ~/.cargo/config.toml
```

Add the config to the registry section:

```yaml
    [registries]
    internal = { index = "sparse+http://localhost:6300/cargo/internal/index/" }
```

You can now use the repository