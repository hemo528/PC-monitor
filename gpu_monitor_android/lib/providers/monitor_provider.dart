import 'dart:async';
import 'package:flutter/foundation.dart';
import '../models/system_data.dart';
import '../services/network_service.dart';

enum AppState { connecting, waitingForData, connected, disconnected }

class MonitorProvider with ChangeNotifier {
  final NetworkService _networkService = NetworkService();

  AppState _state = AppState.connecting;
  SystemData? _currentData;
  final List<SystemData> _history = [];
  String? _error;
  String _ipInput = '';
  String _collectorAddr = '';
  int _styleIndex = 0;

  StreamSubscription<SystemData>? _dataSub;
  StreamSubscription<String>? _errorSub;
  StreamSubscription<bool>? _connSub;

  static const int maxHistory = 120;

  AppState get state => _state;
  SystemData? get currentData => _currentData;
  List<SystemData> get history => List.unmodifiable(_history);
  String? get error => _error;
  String get ipInput => _ipInput;
  String get collectorAddr => _collectorAddr;
  int get styleIndex => _styleIndex;

  MonitorProvider() {
    _dataSub = _networkService.dataStream.listen(_onData);
    _errorSub = _networkService.errorStream.listen(_onError);
    _connSub = _networkService.connectionStream.listen(_onConnection);
    _loadSavedIpAndConnect();
  }

  Future<void> _loadSavedIpAndConnect() async {
    _ipInput = await NetworkService.getLastIp();
    notifyListeners();
    if (_ipInput.trim().isNotEmpty) {
      connect();
    }
  }

  void setIpInput(String ip) {
    _ipInput = ip;
    notifyListeners();
  }

  void setStyleIndex(int i) {
    _styleIndex = i;
    notifyListeners();
  }

  Future<void> connect() async {
    final addr = _ipInput.trim().isEmpty ? '127.0.0.1' : _ipInput.trim();
    _collectorAddr = addr.contains(':') ? addr : '$addr:9876';
    _state = AppState.waitingForData;
    _error = null;
    notifyListeners();

    await NetworkService.saveLastIp(_ipInput);
    await _networkService.connect(_collectorAddr);
  }

  void disconnect() {
    _networkService.disconnect();
    _state = AppState.disconnected;
    _error = 'Disconnected by user';
    notifyListeners();
  }

  void reconnect() {
    connect();
  }

  void backToConnect() {
    _networkService.disconnect();
    _state = AppState.connecting;
    _error = null;
    notifyListeners();
  }

  void _onData(SystemData data) {
    _currentData = data;
    _history.add(data);
    if (_history.length > maxHistory) _history.removeAt(0);
    if (_state != AppState.connected) {
      _state = AppState.connected;
      _error = null;
    }
    notifyListeners();
  }

  void _onError(String err) {
    _error = err;
    _state = AppState.disconnected;
    notifyListeners();
  }

  void _onConnection(bool connected) {
    if (!connected && _state == AppState.connected) {
      _state = AppState.disconnected;
      _error ??= 'Connection lost';
      notifyListeners();
    }
  }

  @override
  void dispose() {
    _dataSub?.cancel();
    _errorSub?.cancel();
    _connSub?.cancel();
    _networkService.dispose();
    super.dispose();
  }
}