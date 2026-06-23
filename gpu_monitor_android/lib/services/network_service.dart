import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:shared_preferences/shared_preferences.dart';
import '../models/monitor_message.dart';
import '../models/system_data.dart';

class NetworkService {
  Socket? _socket;
  StreamSubscription? _subscription;
  String _buffer = '';

  final StreamController<SystemData> _dataController = StreamController<SystemData>.broadcast();
  final StreamController<String> _errorController = StreamController<String>.broadcast();
  final StreamController<bool> _connectionController = StreamController<bool>.broadcast();

  Stream<SystemData> get dataStream => _dataController.stream;
  Stream<String> get errorStream => _errorController.stream;
  Stream<bool> get connectionStream => _connectionController.stream;

  static Future<String> getLastIp() async {
    final prefs = await SharedPreferences.getInstance();
    return prefs.getString('last_ip') ?? '';
  }

  static Future<void> saveLastIp(String ip) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString('last_ip', ip);
  }

  Future<void> connect(String address) async {
    disconnect();

    final addr = address.contains(':') ? address : '$address:9876';
    final parts = addr.split(':');
    final host = parts[0];
    final port = parts.length > 1 ? int.tryParse(parts[1]) ?? 9876 : 9876;

    try {
      _socket = await Socket.connect(host, port, timeout: const Duration(seconds: 3));
      _connectionController.add(true);
      _buffer = '';

      _subscription = _socket!.listen(
        (data) {
          _buffer += utf8.decode(data, allowMalformed: true);
          while (_buffer.contains('\n')) {
            final idx = _buffer.indexOf('\n');
            final line = _buffer.substring(0, idx).trim();
            _buffer = _buffer.substring(idx + 1);
            if (line.isEmpty) continue;
            try {
              final msg = MonitorMessage.fromJson(jsonDecode(line) as Map<String, dynamic>);
              if (msg.type == MessageType.dataUpdate && msg.data != null) {
                _dataController.add(msg.data!);
              } else if (msg.type == MessageType.error) {
                _errorController.add(msg.error ?? 'Unknown error');
              }
            } catch (e) {
              // skip malformed lines
            }
          }
        },
        onError: (e) {
          _errorController.add('Connection error: $e');
          _connectionController.add(false);
        },
        onDone: () {
          _connectionController.add(false);
        },
      );
    } catch (e) {
      _errorController.add('Cannot connect to $addr: $e');
      _connectionController.add(false);
    }
  }

  void disconnect() {
    _subscription?.cancel();
    _socket?.destroy();
    _socket = null;
    _subscription = null;
    _buffer = '';
  }

  void dispose() {
    disconnect();
    _dataController.close();
    _errorController.close();
    _connectionController.close();
  }
}
