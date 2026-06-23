import 'package:flutter/material.dart';
import '../providers/monitor_provider.dart';
import '../themes/app_themes.dart';

class ConnectionScreen extends StatefulWidget {
  final MonitorProvider provider;
  final AppTheme theme;
  const ConnectionScreen({super.key, required this.provider, required this.theme});

  @override
  State<ConnectionScreen> createState() => _ConnectionScreenState();
}

class _ConnectionScreenState extends State<ConnectionScreen> {
  late TextEditingController _ctrl;

  @override
  void initState() {
    super.initState();
    _ctrl = TextEditingController(text: widget.provider.ipInput);
  }

  @override
  void dispose() {
    _ctrl.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final t = widget.theme;
    return Scaffold(
      backgroundColor: t.bgColor,
      body: Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(Icons.monitor_heart_rounded, color: t.titleColor, size: 64),
            const SizedBox(height: 12),
            Text('GPU Monitor', style: TextStyle(color: t.titleColor, fontSize: 36, fontWeight: FontWeight.bold)),
            const SizedBox(height: 6),
            Text('Enter collector address', style: TextStyle(color: t.textColor, fontSize: 16)),
            const SizedBox(height: 24),
            SizedBox(
              width: 300,
              child: TextField(
                controller: _ctrl,
                onChanged: (v) => widget.provider.setIpInput(v),
                style: TextStyle(color: t.valueColor),
                decoration: InputDecoration(
                  hintText: 'e.g. 192.168.1.100',
                  hintStyle: TextStyle(color: t.textColor.withOpacity(0.5)),
                  filled: true,
                  fillColor: t.cardBg,
                  border: OutlineInputBorder(borderRadius: BorderRadius.circular(8), borderSide: BorderSide(color: t.borderColor)),
                  enabledBorder: OutlineInputBorder(borderRadius: BorderRadius.circular(8), borderSide: BorderSide(color: t.borderColor)),
                  focusedBorder: OutlineInputBorder(borderRadius: BorderRadius.circular(8), borderSide: BorderSide(color: t.buttonBg, width: 2)),
                  contentPadding: const EdgeInsets.all(12),
                ),
              ),
            ),
            const SizedBox(height: 14),
            ElevatedButton.icon(
              onPressed: () => widget.provider.connect(),
              icon: const Icon(Icons.power_settings_new_rounded, size: 18),
              label: const Text('Connect', style: TextStyle(fontSize: 16)),
              style: ElevatedButton.styleFrom(backgroundColor: t.buttonBg, foregroundColor: t.valueColor, padding: const EdgeInsets.symmetric(horizontal: 28, vertical: 10), shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8))),
            ),
            if (widget.provider.error != null) ...[
              const SizedBox(height: 12),
              Text(widget.provider.error!, style: TextStyle(color: Colors.red.shade300, fontSize: 13)),
            ],
          ],
        ),
      ),
    );
  }
}