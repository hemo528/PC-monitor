import 'package:flutter/material.dart';
import '../themes/app_themes.dart';
import '../providers/monitor_provider.dart';

class ErrorScreen extends StatelessWidget {
  final MonitorProvider provider;
  final AppTheme theme;
  const ErrorScreen({super.key, required this.provider, required this.theme});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: theme.bgColor,
      body: Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text('Connection Lost', style: TextStyle(color: Colors.red.shade300, fontSize: 32, fontWeight: FontWeight.bold)),
            const SizedBox(height: 14),
            Text(provider.error ?? 'Unknown error', style: TextStyle(color: theme.textColor, fontSize: 18)),
            const SizedBox(height: 8),
            Text('Collector: ${provider.collectorAddr}', style: TextStyle(color: theme.textColor, fontSize: 14)),
            const SizedBox(height: 28),
            Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                ElevatedButton(
                  onPressed: provider.reconnect,
                  style: ElevatedButton.styleFrom(backgroundColor: theme.buttonBg, foregroundColor: theme.valueColor, padding: const EdgeInsets.symmetric(horizontal: 28, vertical: 12), shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8))),
                  child: const Text('Reconnect', style: TextStyle(fontSize: 18)),
                ),
                const SizedBox(width: 16),
                ElevatedButton(
                  onPressed: provider.backToConnect,
                  style: ElevatedButton.styleFrom(backgroundColor: theme.dangerBg, foregroundColor: theme.valueColor, padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 10), shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8))),
                  child: const Text('Change Address', style: TextStyle(fontSize: 14)),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
