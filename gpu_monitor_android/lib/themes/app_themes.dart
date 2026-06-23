import 'package:flutter/material.dart';

/// 应用主题配置 - 与PC版styles.rs保持一致
class AppTheme {
  final String name;
  final Color bgColor;
  final Color cardBg;
  final Color headerBg;
  final Color titleColor;
  final Color textColor;
  final Color valueColor;
  final Color cpuColor;
  final Color memoryColor;
  final Color gpuColor;
  final Color gpuMemoryColor;
  final Color buttonBg;
  final Color buttonHover;
  final Color dangerBg;
  final Color borderColor;

  const AppTheme({
    required this.name,
    required this.bgColor,
    required this.cardBg,
    required this.headerBg,
    required this.titleColor,
    required this.textColor,
    required this.valueColor,
    required this.cpuColor,
    required this.memoryColor,
    required this.gpuColor,
    required this.gpuMemoryColor,
    required this.buttonBg,
    required this.buttonHover,
    required this.dangerBg,
    required this.borderColor,
  });
}

/// 预设主题列表 - 与PC版create_styles()对应
final List<AppTheme> appThemes = [
  // 深色主题（默认）
  const AppTheme(
    name: 'Dark',
    bgColor: Color(0xFF1A1A1F),
    cardBg: Color(0xF226262E),
    headerBg: Color(0xFA1F1F27),
    titleColor: Color(0xFFE5E5F2),
    textColor: Color(0xB3B3B3BF),
    valueColor: Colors.white,
    cpuColor: Color(0xFF4DCC66),
    memoryColor: Color(0xFF4D99FF),
    gpuColor: Color(0xFFE69933),
    gpuMemoryColor: Color(0xFFCC66E6),
    buttonBg: Color(0xFF3380CC),
    buttonHover: Color(0xFF4D99E6),
    dangerBg: Color(0xFFCC3333),
    borderColor: Color(0x804D4D59),
  ),
  
  // 科技蓝主题
  const AppTheme(
    name: 'Tech Blue',
    bgColor: Color(0xFF0D1426),
    cardBg: Color(0xF2142033),
    headerBg: Color(0xFA0F1A2E),
    titleColor: Color(0xFF66CCFF),
    textColor: Color(0xB399B3CC),
    valueColor: Color(0xFF80E6FF),
    cpuColor: Color(0xFF33E699),
    memoryColor: Color(0xFF4DB3FF),
    gpuColor: Color(0xFF00CCE6),
    gpuMemoryColor: Color(0xFF8080FF),
    buttonBg: Color(0xFF1A66B3),
    buttonHover: Color(0xFF3380CC),
    dangerBg: Color(0xFFB32626),
    borderColor: Color(0x80336699),
  ),
  
  // 赛博朋克主题
  const AppTheme(
    name: 'Cyberpunk',
    bgColor: Color(0xFF14051F),
    cardBg: Color(0xF21F0A2E),
    headerBg: Color(0xFA1A0826),
    titleColor: Color(0xFFFF3399),
    textColor: Color(0xB3CC99CC),
    valueColor: Color(0xFFFF66CC),
    cpuColor: Color(0xFF00FFCC),
    memoryColor: Color(0xFFFF00CC),
    gpuColor: Color(0xFFFFFF00),
    gpuMemoryColor: Color(0xFFCC00FF),
    buttonBg: Color(0xFFCC1A66),
    buttonHover: Color(0xFFE63380),
    dangerBg: Color(0xFFE61A1A),
    borderColor: Color(0x80993399),
  ),
  
  // 极简白主题
  const AppTheme(
    name: 'Minimal Light',
    bgColor: Color(0xFFF2F2F5),
    cardBg: Color(0xF2FFFFFF),
    headerBg: Color(0xFAFAFAFC),
    titleColor: Color(0xFF1A1A26),
    textColor: Color(0xB3666673),
    valueColor: Color(0xFF1A1A1A),
    cpuColor: Color(0xFF33B34D),
    memoryColor: Color(0xFF3380CC),
    gpuColor: Color(0xFFCC801A),
    gpuMemoryColor: Color(0xFF994DB3),
    buttonBg: Color(0xFF3380CC),
    buttonHover: Color(0xFF4D99E6),
    dangerBg: Color(0xFFCC3333),
    borderColor: Color(0xCCCCCCCC),
  ),
];
