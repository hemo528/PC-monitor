import 'package:fl_chart/fl_chart.dart';
import 'package:flutter/material.dart';
import '../models/system_data.dart';
import '../themes/app_themes.dart';

class HistoryChart extends StatelessWidget {
  final List<SystemData> history;
  final AppTheme theme;
  const HistoryChart({super.key, required this.history, required this.theme});

  bool get _isLight => (theme.bgColor.red + theme.bgColor.green + theme.bgColor.blue) / 3 > 128;

  @override
  Widget build(BuildContext context) {
    if (history.isEmpty) {
      return Container(
        height: 180,
        padding: const EdgeInsets.all(18),
        decoration: BoxDecoration(color: theme.cardBg, borderRadius: BorderRadius.circular(12), border: Border.all(color: theme.borderColor)),
        child: Center(child: Text('Waiting for data...', style: TextStyle(color: theme.textColor, fontSize: 14))),
      );
    }

    final cpuVals = history.map((d) => d.cpuUsage).toList();
    final memVals = history.map((d) => d.memoryUsage).toList();
    final gpuVals = history.where((d) => d.gpu != null).map((d) => d.gpu!.utilization).toList();

    return Container(
      padding: const EdgeInsets.all(18),
      decoration: BoxDecoration(color: theme.cardBg, borderRadius: BorderRadius.circular(12), border: Border.all(color: theme.borderColor)),
      child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
        Text('History', style: TextStyle(color: theme.titleColor, fontSize: 16, fontWeight: FontWeight.w600)),
        const SizedBox(height: 10),
        _chartRow('CPU History', cpuVals, theme.cpuColor),
        const SizedBox(height: 12),
        _chartRow('Memory History', memVals, theme.memoryColor),
        if (gpuVals.isNotEmpty) ...[
          const SizedBox(height: 12),
          _chartRow('GPU History', gpuVals, theme.gpuColor),
        ],
      ]),
    );
  }

  Widget _chartRow(String title, List<double> values, Color color) {
    return Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
      Text(title, style: TextStyle(color: color, fontSize: 12)),
      const SizedBox(height: 3),
      SizedBox(height: 140, child: _chart(values, color)),
    ]);
  }

  Widget _chart(List<double> values, Color color) {
    if (values.length < 2) {
      return Center(child: Text('Collecting...', style: TextStyle(color: theme.textColor.withOpacity(0.5), fontSize: 12)));
    }
    final maxY = values.reduce((a, b) => a > b ? a : b).clamp(100, double.infinity).toDouble();
    final spots = values.asMap().entries.map((e) => FlSpot(e.key.toDouble(), e.value)).toList();
    final gridColor = _isLight ? const Color(0xFFB3B3BA) : const Color(0xFF292933);
    final axisColor = _isLight ? const Color(0xFF595966) : const Color(0xFF666673);

    return LineChart(LineChartData(
      minY: 0,
      maxY: maxY,
      gridData: FlGridData(show: true, drawVerticalLine: false, horizontalInterval: maxY / 4, getDrawingHorizontalLine: (v) => FlLine(color: gridColor, strokeWidth: 0.5)),
      titlesData: FlTitlesData(
        leftTitles: AxisTitles(sideTitles: SideTitles(showTitles: true, reservedSize: 36, getTitlesWidget: (v, _) => Text(v.toStringAsFixed(0), style: TextStyle(color: axisColor, fontSize: 9)))),
        bottomTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
        topTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
        rightTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
      ),
      borderData: FlBorderData(show: false),
      lineBarsData: [
        LineChartBarData(
          spots: spots,
          isCurved: true,
          color: color,
          barWidth: 1.8,
          isStrokeCapRound: true,
          dotData: FlDotData(show: true, getDotPainter: (s, p, b, i) => FlDotCirclePainter(radius: 1.8, color: color, strokeWidth: 0)),
          belowBarData: BarAreaData(show: true, color: color.withOpacity(0.06)),
        ),
      ],
      lineTouchData: const LineTouchData(enabled: false),
    ));
  }
}
