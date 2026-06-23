class GpuData {
  final String name;
  final double utilization;
  final double temperature;
  final double memoryUtilization;
  final int memoryUsed;
  final int memoryTotal;
  final double powerUsage;
  final int clockSpeed;
  final double fanSpeed;
  final int timestamp;

  GpuData({
    required this.name,
    required this.utilization,
    required this.temperature,
    required this.memoryUtilization,
    required this.memoryUsed,
    required this.memoryTotal,
    required this.powerUsage,
    required this.clockSpeed,
    required this.fanSpeed,
    required this.timestamp,
  });

  factory GpuData.fromJson(Map<String, dynamic> json) {
    return GpuData(
      name: json['name'] as String,
      utilization: (json['utilization'] as num).toDouble(),
      temperature: (json['temperature'] as num).toDouble(),
      memoryUtilization: (json['memory_utilization'] as num).toDouble(),
      memoryUsed: json['memory_used'] as int,
      memoryTotal: json['memory_total'] as int,
      powerUsage: (json['power_usage'] as num).toDouble(),
      clockSpeed: json['clock_speed'] as int,
      fanSpeed: (json['fan_speed'] as num).toDouble(),
      timestamp: json['timestamp'] as int,
    );
  }
}
