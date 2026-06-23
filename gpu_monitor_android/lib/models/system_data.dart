import 'gpu_data.dart';

class SystemData {
  final double cpuUsage;
  final double cpuFrequency;
  final double cpuTemperature;
  final double memoryUsage;
  final int memoryUsed;
  final int memoryTotal;
  final GpuData? gpu;

  SystemData({
    required this.cpuUsage,
    this.cpuFrequency = 0,
    this.cpuTemperature = 0,
    required this.memoryUsage,
    required this.memoryUsed,
    required this.memoryTotal,
    this.gpu,
  });

  factory SystemData.fromJson(Map<String, dynamic> json) {
    return SystemData(
      cpuUsage: (json['cpu_usage'] as num).toDouble(),
      cpuFrequency: (json['cpu_frequency'] as num?)?.toDouble() ?? 0,
      cpuTemperature: (json['cpu_temperature'] as num?)?.toDouble() ?? 0,
      memoryUsage: (json['memory_usage'] as num).toDouble(),
      memoryUsed: json['memory_used'] as int,
      memoryTotal: json['memory_total'] as int,
      gpu: json['gpu'] != null ? GpuData.fromJson(json['gpu'] as Map<String, dynamic>) : null,
    );
  }
}
