# Midway

Midway runs shell commands and sends a completion notification after a command exits if it ran longer than a configured threshold. The default threshold is 10 seconds; change `notify_after.minimum_send_after` in `midway.yaml` to adjust it. Midway does not currently send progress notifications while a command is still running. Notifications can be sent to local desktop notifications and configured messaging services.

## Quick start

Install Midway with Cargo:

```bash
cargo install midway-cli
```

Create a configuration, then run a command through Midway:

```bash
midway init
midway run cargo build
```

To send a notification directly:

```bash
midway send-text --message "Build finished"
```

## Documentation

See the [Midway documentation](https://midway.hazyio.com/) for [installation options](https://midway.hazyio.com/installation), [configuration](https://midway.hazyio.com/config), [commands](https://midway.hazyio.com/commands/init), and [notification setup](https://midway.hazyio.com/notifications/local).

## Development

Install the stable Rust toolchain, then run these checks from the repository root before opening a pull request. All four checks must pass; CI runs the same commands.

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace --release
```

## Roadmap

- [ ] Handle chained commands
- [ ] Microsoft Teams
- [ ] Google Chat
- [ ] Mattermost and Rocket.Chat
- [ ] Zulip
- [ ] Matrix
- [ ] Generic webhook
