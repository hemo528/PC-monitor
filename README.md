# PC Monitor

实时系统性能监控工具，采用采集端 + 显示端分离架构，支持 Windows 和安卓双平台。

## 架构

```
┌──────────────┐   TCP 端口 9876   ┌──────────────┐
│   采集端      │ ────────────────▶ │   显示端      │
│  (Windows)    │   JSON 数据流     │ Windows / 安卓│
└──────────────┘                    └──────────────┘
```

- **采集端** — 运行在被监控的电脑上，通过 sysinfo + NVML 采集 CPU、内存、GPU 数据，通过 TCP 推送
- **显示端** — 连接采集端，实时展示弧形仪表盘、进度条、历史曲线图。支持 Windows 桌面端（Rust + iced）和安卓端（Flutter）

## 功能特性

- 4 个弧形仪表盘：CPU 占用、内存占用、GPU 占用、显存占用
- CPU 实时主频与温度（通过 WMI 获取，与任务管理器一致）
- GPU 温度、功耗、风扇转速、核心频率
- CPU / 内存 / GPU 使用率历史折线图
- 4 种主题：Dark、Tech Blue、Cyberpunk、Minimal Light
- IP 地址记忆与自动连接
- 多客户端同时连接

## 下载

在 [Releases](../../releases) 页面下载：

| 文件 | 大小 | 说明 |
|------|------|------|
| `PC-Monitor-v1.0.zip` | 4 MB | Windows 采集端 + 显示端 |
| `PC-Monitor-Android-v1.0.apk` | 43.7 MB | 安卓显示端 |
| `PC-Monitor-v1.0-完整包.zip` | 24.5 MB | 全部文件 + 使用说明 |

## 快速开始

### 1. 启动采集端

在需要被监控的电脑上双击 `collector.exe`，保持命令行窗口不要关闭。

### 2. 启动显示端

- **Windows**：双击 `display.exe`，输入采集端电脑的 IP 地址，点击 Connect
- **安卓**：安装 APK，输入采集端电脑的 IP 地址，点击 Connect
- IP 地址会自动保存，下次启动自动连接

### 3. 查看 IP 地址

在采集端电脑上按 `Win + R`，输入 `cmd`，执行 `ipconfig`，找到 `IPv4 地址`。

## 从源码构建

### 环境要求

- [Rust 工具链](https://rustup.rs/)（stable）
- Visual Studio C++ Build Tools（Windows）
- [Flutter SDK 3.24+](https://flutter.dev/) + Android SDK（安卓端）

### 构建采集端和显示端

```powershell
cargo build --release
```

产物在 `target/release/collector.exe` 和 `target/release/display.exe`。

### 构建安卓 APK

```powershell
cd gpu_monitor_android
flutter pub get
flutter build apk --release --no-tree-shake-icons
```

产物在 `build/app/outputs/flutter-apk/app-release.apk`。

> **注意**：Flutter 的编译器不支持中文路径，如果项目路径包含中文，请将 `gpu_monitor_android` 文件夹复制到纯英文路径（如 `C:\gpu_monitor_android`）再构建。

## 项目结构

```
├── Cargo.toml                      # Rust 项目配置
├── src/
│   ├── shared.rs                   # 共享数据类型
│   ├── collector/
│   │   ├── main.rs                 # TCP 服务端，系统数据采集
│   │   └── gpu.rs                  # NVIDIA GPU 数据采集（NVML）
│   └── display/
│       ├── main.rs                 # iced GUI 应用入口
│       ├── network.rs              # TCP 客户端，自动重连
│       └── ui/
│           ├── mod.rs              # 页面视图
│           ├── styles.rs           # 主题定义
│           └── dashboard.rs        # 仪表盘、详情面板、图表
└── gpu_monitor_android/            # Flutter 安卓端
    ├── lib/
    │   ├── main.dart               # 应用入口
    │   ├── models/                 # 数据模型
    │   ├── providers/              # 状态管理
    │   ├── screens/                # 页面
    │   ├── services/               # TCP 网络服务
    │   ├── themes/                 # 4 种主题
    │   └── widgets/                # 仪表盘、面板、图表组件
    └── pubspec.yaml
```

## 网络协议

采集端监听 TCP 端口 9876，发送换行分隔的 JSON 数据：

```json
{"DataUpdate":{"cpu_usage":20.5,"cpu_frequency":4893.0,"cpu_temperature":27.8,"memory_usage":69.4,"memory_used":11222,"memory_total":16159,"gpu":{"name":"NVIDIA GeForce RTX 4070 Laptop","utilization":12.0,"temperature":42.0,"memory_utilization":8.5,"memory_used":700,"memory_total":8192,"power_usage":35.2,"clock_speed":2100,"fan_speed":30.0,"timestamp":1719139200}}}
```

## 性能开销

| 组件 | CPU 占用 | 内存占用 |
|------|---------|---------|
| 采集端 | ~0.9% | ~40 MB |
| 显示端（Windows） | ~0.02% | ~27 MB |

## 常见问题

**连接不上？** 确认采集端已启动，两台设备在同一局域网，防火墙放行 9876 端口。

**GPU 数据不显示？** 需要 NVIDIA 显卡及驱动。AMD/Intel 核显暂不支持 GPU 监控。

**CPU 温度显示 N/A？** 部分电脑的 WMI 温度传感器不可用，属正常现象。

## 开源协议

MIT