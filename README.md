# Midway

Midway runs shell commands and sends a completion notification after a command runs for at least a configured threshold (10 seconds by default). It can also send notifications while commands run. Configure `command_notifications` in `midway.yaml` to control notification timing. Notifications can be sent to local desktop notifications and configured messaging services.

## Quick start

Install Midway with Cargo:

```bash
cargo install midway-cli
```

Prebuilt releases are available for Linux x86_64, Windows x86_64, and Apple Silicon macOS (aarch64). On ARM64 Windows, ARM64 Linux, or Intel macOS (x86_64), install Rust and Cargo and use the command above to build Midway for your machine.

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
