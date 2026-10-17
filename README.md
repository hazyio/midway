# Midway

Midway runs shell commands and can send periodic notifications while they run, followed by a completion notification. Running notifications are enabled by default and repeat every 15 seconds; completion notifications are sent for commands that run at least 10 seconds. Configure `command_notifications` in `midway.yaml` to change this behavior. Notifications can be sent to local desktop notifications and configured messaging services.

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

Install the stable Rust toolchain with the `rustfmt` and `clippy` components, then run this from the repository root before opening a pull request. It runs the same checks as CI.

```bash
./scripts/check.sh
```

## Roadmap

- [ ] Handle log_lines_to_include in [notify_after](crates/config/src/utils/notify_after.rs#L14)
- [ ] Handle chained commands
- [ ] Microsoft Teams
- [ ] Google Chat
- [ ] Mattermost and Rocket.Chat
- [ ] Zulip
- [ ] Matrix
- [ ] Generic webhook
