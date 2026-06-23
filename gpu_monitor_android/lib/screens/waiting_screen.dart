import 'package:flutter/material.dart';
import '../themes/app_themes.dart';

class WaitingScreen extends StatelessWidget {
  final String addr;
  final AppTheme theme;
  final VoidCallback onCancel;
  const WaitingScreen({super.key, required this.addr, required this.theme, required this.onCancel});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: theme.bgColor,
      body: Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text('Connecting...', style: TextStyle(color: theme.titleColor, fontSize: 32, fontWeight: FontWeight.bold)),
            const SizedBox(height: 14),
            Text('Waiting for data from $addr', style: TextStyle(color: theme.textColor, fontSize: 18)),
            const SizedBox(height: 28),
            ElevatedButton(
              onPressed: onCancel,
              style: ElevatedButton.styleFrom(backgroundColor: theme.dangerBg, foregroundColor: theme.valueColor, padding: const EdgeInsets.symmetric(horizontal: 28, vertical: 12), shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8))),
              child: const Text('Cancel', style: TextStyle(fontSize: 18)),
            ),
          ],
        ),
      ),
    );
  }
}
