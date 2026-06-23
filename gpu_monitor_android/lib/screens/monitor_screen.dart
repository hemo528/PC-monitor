import 'package:flutter/material.dart';
import '../models/system_data.dart';
import '../models/gpu_data.dart';
import '../providers/monitor_provider.dart';
import '../themes/app_themes.dart';
import '../widgets/arc_gauge.dart';
import '../widgets/detail_panel.dart';
import '../widgets/history_chart.dart';

class MonitorScreen extends StatelessWidget {
  final MonitorProvider provider;
  final AppTheme theme;
  const MonitorScreen({super.key, required this.provider, required this.theme});

  @override
  Widget build(BuildContext context) {
    final data = provider.currentData!;
    return Scaffold(
      backgroundColor: theme.bgColor,
      body: Column(
        children: [
          _buildHeader(context),
          Expanded(
            child: SingleChildScrollView(
              padding: const EdgeInsets.all(12),
              child: Column(
                children: [
                  _buildGaugeRow(data),
                  const SizedBox(height: 12),
                  _buildDetailPanels(data),
                  const SizedBox(height: 12),
                  HistoryChart(history: provider.history, theme: theme),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildHeader(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
      decoration: BoxDecoration(color: theme.headerBg, border: Border(bottom: BorderSide(color: theme.borderColor))),
      child: Row(children: [
        Icon(Icons.insights_rounded, color: theme.titleColor, size: 22),
        const SizedBox(width: 6),
        Text('Monitor', style: TextStyle(color: theme.titleColor, fontSize: 15, fontWeight: FontWeight.bold)),
        const SizedBox(width: 6),
        Expanded(
          child: Text(
            provider.collectorAddr,
            style: TextStyle(color: theme.textColor, fontSize: 10),
            overflow: TextOverflow.ellipsis,
          ),
        ),
        _iconBtn(Icons.palette_outlined, theme.buttonBg, () => provider.setStyleIndex((provider.styleIndex + 1) % appThemes.length)),
        const SizedBox(width: 4),
        _iconBtn(Icons.link_off_rounded, theme.dangerBg, provider.disconnect),
      ]),
    );
  }

  Widget _iconBtn(IconData icon, Color bg, VoidCallback onTap) {
    return SizedBox(
      width: 30,
      height: 30,
      child: Material(
        color: bg,
        borderRadius: BorderRadius.circular(6),
        child: InkWell(
          borderRadius: BorderRadius.circular(6),
          onTap: onTap,
          child: Icon(icon, color: theme.valueColor, size: 16),
        ),
      ),
    );
  }

  Widget _buildGaugeRow(SystemData data) {
    final gauges = <Widget>[
      Expanded(child: ArcGauge(label: 'CPU', detail: '${data.cpuFrequency.toStringAsFixed(0)} MHz', pct: data.cpuUsage, color: theme.cpuColor, theme: theme)),
      const SizedBox(width: 8),
      Expanded(child: ArcGauge(label: 'Memory', detail: '${data.memoryUsed}/${data.memoryTotal} MB', pct: data.memoryUsage, color: theme.memoryColor, theme: theme)),
    ];
    if (data.gpu != null) {
      gauges.addAll([
        const SizedBox(width: 8),
        Expanded(child: ArcGauge(label: 'GPU', detail: '${data.gpu!.utilization.toStringAsFixed(1)}%', pct: data.gpu!.utilization, color: theme.gpuColor, theme: theme)),
        const SizedBox(width: 8),
        Expanded(child: ArcGauge(label: 'VRAM', detail: '${data.gpu!.memoryUsed}/${data.gpu!.memoryTotal} MB', pct: data.gpu!.memoryUtilization, color: theme.gpuMemoryColor, theme: theme)),
      ]);
    }
    return Row(children: gauges);
  }

  Widget _buildDetailPanels(SystemData data) {
    return Row(crossAxisAlignment: CrossAxisAlignment.start, children: [
      Expanded(child: _cpuPanel(data)),
      const SizedBox(width: 12),
      Expanded(child: data.gpu != null ? _gpuPanel(data.gpu!) : _noGpuCard()),
    ]);
  }

  Widget _cpuPanel(SystemData data) {
    final tempColor = data.cpuTemperature > 85 ? Colors.red.shade300 : data.cpuTemperature > 65 ? Colors.orange : data.cpuTemperature > 0 ? Colors.green.shade300 : theme.textColor;
    return DetailPanel(
      title: 'CPU Details',
      theme: theme,
      items: [
        DetailItem(label: 'Frequency', value: '${data.cpuFrequency.toStringAsFixed(0)} MHz', pct: (data.cpuFrequency / 8000 * 100).clamp(0, 100), color: const Color(0xFF4DCCFF)),
        DetailItem(label: 'Temperature', value: data.cpuTemperature > 0 ? '${data.cpuTemperature.toStringAsFixed(0)}°C' : 'N/A', pct: data.cpuTemperature > 0 ? data.cpuTemperature.clamp(0, 100) : 0, color: tempColor),
      ],
    );
  }

  Widget _gpuPanel(GpuData gpu) {
    final tempColor = gpu.temperature > 80 ? Colors.red.shade300 : gpu.temperature > 60 ? Colors.orange : Colors.green.shade300;
    return DetailPanel(
      title: 'GPU: ${gpu.name.replaceAll('NVIDIA ', '')}',
      trailing: 'Clock: ${gpu.clockSpeed} MHz',
      theme: theme,
      items: [
        DetailItem(label: 'Temperature', value: '${gpu.temperature.toStringAsFixed(0)}°C', pct: gpu.temperature.clamp(0, 100), color: tempColor),
        DetailItem(label: 'Power', value: '${gpu.powerUsage.toStringAsFixed(1)}W', pct: (gpu.powerUsage / 300 * 100).clamp(0, 100), color: const Color(0xFFFFCC33)),
        DetailItem(label: 'Fan', value: '${gpu.fanSpeed.toStringAsFixed(0)}%', pct: gpu.fanSpeed.clamp(0, 100), color: const Color(0xFF66CCFF)),
      ],
    );
  }

  Widget _noGpuCard() {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(color: theme.cardBg, borderRadius: BorderRadius.circular(12), border: Border.all(color: theme.borderColor)),
      child: Center(child: Text('No GPU detected', style: TextStyle(color: theme.textColor, fontSize: 14))),
    );
  }
}