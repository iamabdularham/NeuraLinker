# NeuraLinker

**NeuraLinker** is a Rust-based smart desktop command center inspired by JARVIS. It offers real-time system monitoring, a smart app launcher, and an embedded terminal within a sleek GUI. Primarily designed for macOS, with plans for cross-platform support, it helps automate tasks and control your system efficiently.

---

## Progress So Far

- Real-time CPU, Memory, and Network monitoring.
- Smart launcher interface with view switching.
- Embedded terminal running a Unix shell ("sh") for command execution.
- GUI built with Rust's `eframe` and `egui`.
- Asynchronous communication between terminal and shell using Rust channels.
- Code cleaned up to remove warnings and ensure smooth compilation.

---

## How It Works

- Native window with sidebar for CPU, Memory, Network, and Terminal views.
- System stats refresh every 2 seconds using `sysinfo`.
- Terminal spawns a shell process and communicates asynchronously.
- Network view shows interface data usage; ping and WiFi SSID planned.

---

## Deployment and Compilation

1. Install Rust from [https://rustup.rs](https://rustup.rs).
2. Clone the repo:
   ```
   git clone https://github.com/yourusername/NeuraLinker.git
   cd NeuraLinker
   ```
3. Build and run:
   ```
   cargo run
   ```
4. The app window will launch with system stats and embedded terminal.

---

## Adding New Features

- Add new modules under `src/`.
- Follow existing patterns using `eframe` and `sysinfo`.
- Integrate new features into the main app's view enum and update logic.
- Extend terminal features in `src/terminal/terminal.rs`.
- Contributions welcome via pull requests.

---

## Missing Features & Future Plans

- Terminal improvements (shell integration, history).
- Notifications engine.
- Voice command engine.
- Plugin system.
- Cross-platform support.
- Enhanced network view (ping, WiFi SSID).
- UI/UX polish and theming.

Contributions and ideas are appreciated!

---

## License

MIT License

---

## Author

**Abdul Arham**  
> Twitter: [@yourhandle] • GitHub: [yourusername]
