# doty

A WIP CLI application to manage and handle `.dotfiles`.

## TODOs

- [ ] Documentation via Website/Wiki?

## Ideas

Here are some potential ideas to extend `doty` functionality. 

### Tasks

```toml
[tasks]

[tasks.git.clone]
url = 'git@github.com:android10/doty.git'
destination = '/home/fernando/dev/src'
```

### Rules

```toml
[rules]

[rules.copy.ssh]
origin = "system/ssh/"
destination = "/home/fernando/.ssh/"

[rules.copy.wireguard]
origin = "system/wg/"
destination = "/etc/wireguard/"
```

### Plugins

```toml
[plugins]
```

## Special Thanks and References

 - [Rust CLI Working Group](https://github.com/rust-cli).
 - [Rust CLI Book](https://rust-cli.github.io/book/index.html).

