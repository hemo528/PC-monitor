import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:provider/provider.dart';
import 'providers/monitor_provider.dart';
import 'themes/app_themes.dart';
import 'screens/connection_screen.dart';
import 'screens/waiting_screen.dart';
import 'screens/monitor_screen.dart';
import 'screens/error_screen.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  SystemChrome.setEnabledSystemUIMode(SystemUiMode.immersiveSticky);
  SystemChrome.setPreferredOrientations([DeviceOrientation.portraitUp, DeviceOrientation.landscapeLeft, DeviceOrientation.landscapeRight]);
  runApp(
    ChangeNotifierProvider(
      create: (_) => MonitorProvider(),
      child: const GpuMonitorApp(),
    ),
  );
}

class GpuMonitorApp extends StatelessWidget {
  const GpuMonitorApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'PC Monitor',
      debugShowCheckedModeBanner: false,
      theme: ThemeData.dark(),
      home: const MonitorHomePage(),
    );
  }
}

class MonitorHomePage extends StatelessWidget {
  const MonitorHomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Consumer<MonitorProvider>(
      builder: (context, p, _) {
        final theme = appThemes[p.styleIndex % appThemes.length];
        switch (p.state) {
          case AppState.connecting:
            return ConnectionScreen(provider: p, theme: theme);
          case AppState.waitingForData:
            return WaitingScreen(addr: p.collectorAddr, theme: theme, onCancel: p.backToConnect);
          case AppState.connected:
            if (p.currentData == null) {
              return WaitingScreen(addr: p.collectorAddr, theme: theme, onCancel: p.backToConnect);
            }
            return MonitorScreen(provider: p, theme: theme);
          case AppState.disconnected:
            return ErrorScreen(provider: p, theme: theme);
        }
      },
    );
  }
}