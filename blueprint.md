# NeuraLinker Blueprint (New)

## Modules

- Core Engine: Main loop updating system stats and dispatching events.
- GUI Renderer: Draws stats, buttons, and widgets.
- Launcher: Opens apps via shortcuts.
- Terminal Handler: Embedded terminal emulator.
- Network Monitor: Shows IPs, ping results, WiFi SSID.
- Notification System: Schedules and displays reminders.
- Voice Engine: Converts speech to shell commands (planned).
- Plugin Loader: Loads user-created plugins.

## Technologies

- GUI: `eframe` / `egui`
- System Info: `sysinfo`
- Terminal: `crossterm`
- Configuration: `serde`, `ron`
- Networking: Native Rust libs or `pnet`
- Speech: Optional `coqui-stt`

## Project Structure

```
NeuraLinker/
├── src/
│   ├── core/
│   ├── gui/
│   ├── launcher/
│   ├── terminal/
│   ├── network/
│   ├── notifications/
│   ├── plugins/
│   └── main.rs
├── Cargo.toml
└── README.md
```

## Key Concepts

- Event-driven architecture with message bus.
- Asynchronous background tasks.
- Modular codebase for easy extension.

## Timeline

1. Core Engine + GUI prototype
2. App Launcher + System Monitor
3. Embedded Terminal + Notifications
4. Plugin Support + Extensions
5. Voice Commands + Polish (Bonus)

## Future Ideas

- Profiles for different work modes.
- AI suggestions for optimizations.
- Encrypted configuration files.

---

*NeuraLinker is your mind, digitized.*
