import 'package:flutter_test/flutter_test.dart';
import 'package:provider/provider.dart';
import 'package:gpu_monitor_android/main.dart';
import 'package:gpu_monitor_android/providers/monitor_provider.dart';

void main() {
  testWidgets('App launches and shows connection screen', (WidgetTester tester) async {
    await tester.pumpWidget(
      ChangeNotifierProvider(
        create: (_) => MonitorProvider(),
        child: const GpuMonitorApp(),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.text('PC Monitor'), findsOneWidget);
  });
}