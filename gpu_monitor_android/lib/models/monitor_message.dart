import 'system_data.dart';

enum MessageType { heartbeat, dataUpdate, error }

class MonitorMessage {
  final MessageType type;
  final SystemData? data;
  final String? error;

  MonitorMessage({required this.type, this.data, this.error});

  factory MonitorMessage.fromJson(Map<String, dynamic> json) {
    final typeStr = json.keys.first;
    switch (typeStr) {
      case 'DataUpdate':
        return MonitorMessage(
          type: MessageType.dataUpdate,
          data: SystemData.fromJson(json['DataUpdate'] as Map<String, dynamic>),
        );
      case 'Error':
        return MonitorMessage(type: MessageType.error, error: json['Error'] as String);
      case 'Heartbeat':
      default:
        return MonitorMessage(type: MessageType.heartbeat);
    }
  }
}
