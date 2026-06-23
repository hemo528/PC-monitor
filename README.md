# PC Monitor

Real-time CPU / Memory / GPU performance monitoring tool with collector-display architecture. The **collector** runs on the target Windows machine, the **display** connects remotely over TCP to visualize live data.

## Architecture

```
┌─────────────┐   TCP (port 9876)   ┌─────────────┐
│  Collector   │ ──────────────────▶ │   Display    │
│  (Win/Linux) │   JSON lines        │  Win / Android│
└─────────────┘                      └─────────────┘
```

- **Collector** — lightweight service that reads CPU, memory, and NVIDIA GPU metrics via `sysinfo` + NVML, then streams JSON over TCP.
- **Display** — connects to the collector, shows live arc gauges, progress bars, and history charts. Available as a Windows desktop app (iced GUI) and an Android APK (Flutter).

## Features

- 4 arc gauges: CPU %, Memory %, GPU %, VRAM %
- CPU frequency & temperature (via WMI), GPU temperature / power / fan speed
- Real-time history line charts (CPU, Memory, GPU)
- 4 themes: Dark, Tech Blue, Cyberpunk, Minimal Light
- IP address persistence & auto-connect
- Cross-platform display: Windows + Android

## Screenshots

> Add screenshots here after running the app

## Downloads

Pre-built binaries are available in the [Releases](../../releases) page:

| File | Description |
|------|-------------|
| `PC-Monitor-v1.0.zip` | Windows collector + display (`collector.exe` + `display.exe`) |
| `PC-Monitor-Android-v1.0.apk` | Android display app |

## Build from Source

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (stable)
- Visual Studio C++ Build Tools (Windows)
- [Flutter SDK 3.24+](https://flutter.dev/) + Android SDK (for Android APK)

### Collector + Display (Windows)

```powershell
# Build both binaries
cargo build --release

# Run collector (on the machine to monitor)
.\target\release\collector.exe

# Run display (on the same or another machine)
.\target\release\display.exe
```

### Android APK

```powershell
cd gpu_monitor_android
flutter pub get

# Build must be on an ASCII-only path due to Flutter impellerc limitation
flutter build apk --release --no-tree-shake-icons

# APK output:
# build/app/outputs/flutter-apk/app-release.apk
```

> **Note:** If your project path contains non-ASCII characters (e.g. Chinese), copy the `gpu_monitor_android` folder to a pure-ASCII path like `C:\gpu_monitor_android` before building.

## Project Structure

```
├── Cargo.toml                  # Rust workspace config
├── src/
│   ├── shared.rs               # Shared data types (SystemData, GpuData, MonitorMessage)
│   ├── collector/
│   │   └── main.rs             # TCP server, system data collection
│   └── display/
│       ├── main.rs             # iced GUI application
│       ├── network.rs          # TCP client with auto-reconnect
│       └── ui/
│           ├── mod.rs          # View functions (connection, monitor, error)
│           ├── styles.rs       # Theme definitions
│           └── dashboard.rs    # Arc gauges, detail panels, history charts
└── gpu_monitor_android/        # Flutter Android display app
    ├── lib/
    │   ├── main.dart
    │   ├── models/             # SystemData, GpuData, MonitorMessage
    │   ├── providers/          # State management (MonitorProvider)
    │   ├── screens/            # Connection, monitor, error screens
    │   ├── services/           # TCP network service
    │   ├── themes/             # 4 color themes
    │   └── widgets/            # Arc gauge, detail panel, history chart
    └── pubspec.yaml
```

## Configuration

The display app saves the last connected IP to `gpu_monitor_display.json` next to the executable (Windows) or via `SharedPreferences` (Android). On next launch it auto-connects.

## Network Protocol

The collector listens on **TCP port 9876** and sends newline-delimited JSON:

```json
{"DataUpdate":{"cpu_usage":20.5,"cpu_frequency":4893.0,"cpu_temperature":27.8,"memory_usage":69.4,"memory_used":11222,"memory_total":16159,"gpu":{"name":"NVIDIA GeForce RTX 4070 Laptop","utilization":12.0,"temperature":42.0,"memory_utilization":8.5,"memory_used":700,"memory_total":8192,"power_usage":35.2,"clock_speed":2100,"fan_speed":30.0,"timestamp":1719139200}}}
{"Heartbeat"}
```

## Performance Overhead

- **Collector:** ~0.9% CPU, ~40 MB RAM
- **Display (Windows):** ~0.02% CPU, ~27 MB RAM

## License

MIT