import 'dart:math';
import 'package:flutter/material.dart';
import '../themes/app_themes.dart';

class ArcGauge extends StatelessWidget {
  final String label;
  final String detail;
  final double pct;
  final Color color;
  final AppTheme theme;

  const ArcGauge({super.key, required this.label, required this.detail, required this.pct, required this.color, required this.theme});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(8),
      decoration: BoxDecoration(color: theme.cardBg, borderRadius: BorderRadius.circular(12), border: Border.all(color: theme.borderColor)),
      child: AspectRatio(
        aspectRatio: 1,
        child: LayoutBuilder(builder: (context, constraints) {
          return CustomPaint(
            painter: _ArcPainter(
              pct: pct.clamp(0, 100),
              color: color,
              label: label,
              detail: detail,
              textColor: theme.valueColor,
              secondaryColor: theme.textColor,
              bgColor: _isLight ? const Color(0x66BFBFC7) : const Color(0x99292933),
            ),
            size: Size(constraints.maxWidth, constraints.maxHeight),
          );
        }),
      ),
    );
  }

  bool get _isLight => (theme.bgColor.red + theme.bgColor.green + theme.bgColor.blue) / 3 > 128;
}

class _ArcPainter extends CustomPainter {
  final double pct;
  final Color color;
  final String label;
  final String detail;
  final Color textColor;
  final Color secondaryColor;
  final Color bgColor;

  _ArcPainter({required this.pct, required this.color, required this.label, required this.detail, required this.textColor, required this.secondaryColor, required this.bgColor});

  @override
  void paint(Canvas canvas, Size size) {
    final side = size.shortestSide;
    final cx = size.width / 2;
    final cy = size.height / 2 - side * 0.06;
    final radius = side * 0.36;
    final strokeWidth = (radius * 0.14).clamp(4.0, 16.0);

    const startAngle = pi * 135 / 180;
    const sweepAngle = pi * 270 / 180;

    final bgPaint = Paint()
      ..color = bgColor
      ..style = PaintingStyle.stroke
      ..strokeWidth = strokeWidth
      ..strokeCap = StrokeCap.round;

    canvas.drawArc(Rect.fromCircle(center: Offset(cx, cy), radius: radius), startAngle, sweepAngle, false, bgPaint);

    if (pct > 0.2) {
      final valPaint = Paint()
        ..color = color
        ..style = PaintingStyle.stroke
        ..strokeWidth = strokeWidth
        ..strokeCap = StrokeCap.round;
      canvas.drawArc(Rect.fromCircle(center: Offset(cx, cy), radius: radius), startAngle, sweepAngle * pct / 100, false, valPaint);
    }

    _drawCenter(canvas, cx, cy - side * 0.02, '${pct.toStringAsFixed(1)}%', (side * 0.14).clamp(12, 32), textColor);
    _drawCenter(canvas, cx, cy + radius * 0.25, detail, (side * 0.065).clamp(8, 16), secondaryColor);
    _drawCenter(canvas, cx, size.height - side * 0.05, label, (side * 0.075).clamp(9, 18), secondaryColor);
  }

  void _drawCenter(Canvas canvas, double x, double y, String text, double size, Color color) {
    final tp = TextPainter(
      text: TextSpan(text: text, style: TextStyle(color: color, fontSize: size, fontWeight: FontWeight.w600)),
      textDirection: TextDirection.ltr,
    )..layout();
    tp.paint(canvas, Offset(x - tp.width / 2, y - tp.height / 2));
  }

  @override
  bool shouldRepaint(covariant _ArcPainter old) => old.pct != pct || old.color != color;
}
