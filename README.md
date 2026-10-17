# Midway

Midway runs shell commands and notifies you when they take longer than a configured threshold. By default, the threshold is 10 seconds. Notifications can be sent to local desktop notifications and configured messaging services.

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

## Roadmap

- [ ] Handle chained commands
- [ ] Microsoft Teams
- [ ] Google Chat
- [ ] Mattermost and Rocket.Chat
- [ ] Zulip
- [ ] Matrix
- [ ] Generic webhook
