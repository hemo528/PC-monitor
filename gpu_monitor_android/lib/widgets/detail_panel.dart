import 'package:flutter/material.dart';
import '../themes/app_themes.dart';

class DetailItem {
  final String label;
  final String value;
  final double pct;
  final Color color;
  DetailItem({required this.label, required this.value, required this.pct, required this.color});
}

class DetailPanel extends StatelessWidget {
  final String title;
  final String? trailing;
  final AppTheme theme;
  final List<DetailItem> items;

  const DetailPanel({super.key, required this.title, this.trailing, required this.theme, required this.items});

  bool get _isLight => (theme.bgColor.red + theme.bgColor.green + theme.bgColor.blue) / 3 > 128;

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(color: theme.cardBg, borderRadius: BorderRadius.circular(12), border: Border.all(color: theme.borderColor)),
      child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
        Row(children: [
          Text(title, style: TextStyle(color: theme.titleColor, fontSize: 15, fontWeight: FontWeight.w600)),
          if (trailing != null) ...[const Spacer(), Text(trailing!, style: TextStyle(color: theme.textColor, fontSize: 12))],
        ]),
        const SizedBox(height: 12),
        ...items.map((item) => Padding(
          padding: const EdgeInsets.only(bottom: 10),
          child: Column(children: [
            Row(children: [
              Text(item.label, style: TextStyle(color: theme.textColor, fontSize: 12)),
              const Spacer(),
              Text(item.value, style: TextStyle(color: theme.valueColor, fontSize: 13, fontWeight: FontWeight.w600)),
            ]),
            const SizedBox(height: 5),
            ClipRRect(
              borderRadius: BorderRadius.circular(4),
              child: LinearProgressIndicator(
                value: item.pct.clamp(0, 100) / 100,
                minHeight: 7,
                backgroundColor: _isLight ? const Color(0xFFD2D2D7) : const Color(0xFF24242E),
                valueColor: AlwaysStoppedAnimation(item.color),
              ),
            ),
          ]),
        )),
      ]),
    );
  }
}
