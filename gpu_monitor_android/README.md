# GPU Monitor Android Application

## 项目概述

这是一个与PC版功能和界面保持一致的Android应用程序，用于实时监控GPU、CPU和内存使用情况。

## 功能特性

### 核心功能
- **实时数据监控**：接收并显示GPU、CPU、内存使用率
- **仪表盘展示**：进度条式指标卡片，直观显示各项数据
- **历史图表**：使用fl_chart绘制折线图，展示历史趋势
- **样式切换**：4套预设主题（Dark、Tech Blue、Cyberpunk、Minimal Light）
- **响应式设计**：适配不同Android设备屏幕尺寸

### 与PC版一致性
- **数据结构**：完全兼容PC版的SystemData和GpuData格式
- **网络协议**：使用相同的UDP广播协议（端口9876）
- **UI布局**：保持与PC版相同的视觉风格和交互逻辑
- **主题系统**：4套主题颜色配置与PC版完全一致

## 技术栈

- **框架**：Flutter 3.x
- **语言**：Dart
- **状态管理**：Provider
- **图表库**：fl_chart
- **网络通信**：dart:io (RawDatagramSocket)

## 项目结构

```
gpu_monitor_android/
├── lib/
│   ├── main.dart                    # 应用入口
│   ├── models/                      # 数据模型
│   │   ├── gpu_data.dart           # GPU数据结构
│   │   ├── system_data.dart        # 系统数据结构
│   │   └── monitor_message.dart    # 网络消息格式
│   ├── services/                    # 服务层
│   │   └── network_service.dart    # UDP网络服务
│   ├── providers/                   # 状态管理
│   │   └── monitor_provider.dart   # 监控数据提供者
│   ├── screens/                     # 界面
│   │   ├── connection_screen.dart  # 连接界面
│   │   ├── monitor_screen.dart     # 监控界面
│   │   └── error_screen.dart       # 错误界面
│   ├── widgets/                     # 组件
│   │   ├── gauge_card.dart         # 仪表盘卡片
│   │   ├── gpu_dashboard.dart      # GPU仪表盘
│   │   ├── system_dashboard.dart   # 系统仪表盘
│   │   └── history_chart.dart      # 历史图表
│   └── themes/                      # 主题
│       └── app_themes.dart         # 4套预设主题
├── android/                         # Android配置
│   ├── app/
│   │   └── src/
│   │       └── main/
│   │           ├── AndroidManifest.xml
│   │           ├── kotlin/
│   │           └── res/
│   ├── build.gradle
│   └── settings.gradle
├── pubspec.yaml                     # Flutter依赖配置
└── analysis_options.yaml            # 代码分析配置
```

## 环境要求

### 开发环境
- Flutter SDK 3.x 或更高版本
- Dart SDK 3.0 或更高版本
- Android Studio 或 VS Code
- Android SDK (API level 21+)

### Android设备要求
- Android 5.0 (API level 21) 或更高版本
- 支持UDP网络通信
- 建议使用Android 8.0+以获得最佳体验

## 构建步骤

### 1. 安装Flutter SDK

```bash
# 下载Flutter SDK
# https://flutter.dev/docs/get-started/install

# 验证安装
flutter doctor
```

### 2. 配置Android开发环境

```bash
# 安装Android Studio
# 配置Android SDK
# 设置环境变量ANDROID_HOME
```

### 3. 获取依赖

```bash
cd gpu_monitor_android
flutter pub get
```

### 4. 运行调试版本

```bash
# 连接Android设备或启动模拟器
flutter run
```

### 5. 构建发布版本APK

```bash
# 构建未签名APK
flutter build apk --release

# APK位置：build/app/outputs/flutter-apk/app-release.apk
```

### 6. 生成签名APK

#### 方法一：使用Android Studio
1. 打开项目在Android Studio中
2. 选择 Build → Generate Signed Bundle / APK
3. 选择 APK
4. 创建或选择密钥库
5. 选择 release 构建类型
6. 点击 Finish

#### 方法二：使用命令行
```bash
# 1. 生成密钥库
keytool -genkey -v -keystore ~/gpu-monitor-key.jks -keyalg RSA -keysize 2048 -validity 10000 -alias gpu-monitor

# 2. 创建key.properties文件
# 在android目录下创建key.properties：
# storePassword=<your-password>
# keyPassword=<your-password>
# keyAlias=gpu-monitor
# storeFile=<path-to-your-jks-file>

# 3. 配置build.gradle
# 在android/app/build.gradle中添加签名配置

# 4. 构建签名APK
flutter build apk --release
```

## 使用说明

### 1. 启动检测端（PC）
```bash
# 在PC上运行检测端程序
./collector.exe
```

### 2. 启动显示端（Android）
1. 安装APK到Android设备
2. 打开应用
3. 点击"Connect"按钮开始监听UDP广播
4. 等待接收检测端数据

### 3. 功能操作
- **样式切换**：点击顶部"Style: Dark"按钮切换主题
- **断开连接**：点击"Disconnect"按钮停止接收数据
- **重新连接**：在错误界面点击"Retry"按钮

## 网络配置

### 防火墙设置
确保PC端防火墙允许UDP端口9876的出站通信：
```bash
# Windows
netsh advfirewall firewall add rule name="GPU Monitor" dir=out action=allow protocol=UDP localport=9876

# Linux
sudo ufw allow out 9876/udp
```

### 网络要求
- PC和Android设备必须在同一局域网内
- 支持UDP广播通信
- 建议使用Wi-Fi连接以获得最佳性能

## 故障排除

### 1. 无法接收数据
- 检查PC和Android设备是否在同一网络
- 验证防火墙设置
- 确认检测端程序正在运行

### 2. 应用崩溃
- 检查Android版本是否符合要求
- 查看logcat日志获取详细错误信息
- 确保有足够的存储空间

### 3. 性能问题
- 减少历史数据长度（修改maxHistoryLength）
- 降低数据更新频率
- 关闭不必要的后台应用

## 开发说明

### 添加新功能
1. 在models目录添加数据模型
2. 在services目录实现业务逻辑
3. 在providers目录添加状态管理
4. 在screens和widgets目录实现UI

### 自定义主题
修改`lib/themes/app_themes.dart`中的颜色配置：
```dart
const AppTheme(
  name: 'Custom',
  bgColor: Color(0xFFYourColor),
  // ... 其他颜色配置
);
```

## 版本历史

- **v1.0.0** (2026-06-12)
  - 初始版本发布
  - 实现与PC版一致的核心功能
  - 支持4套主题样式
  - 响应式布局设计

## 许可证

本项目基于PC版GPU监控应用开发，遵循相同的许可证协议。

## 联系方式

如有问题或建议，请联系开发团队。
