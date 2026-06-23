# PC Monitor Android Application

## 椤圭洰姒傝堪

杩欐槸涓€涓笌PC鐗堝姛鑳藉拰鐣岄潰淇濇寔涓€鑷寸殑Android搴旂敤绋嬪簭锛岀敤浜庡疄鏃剁洃鎺PU銆丆PU鍜屽唴瀛樹娇鐢ㄦ儏鍐点€?
## 鍔熻兘鐗规€?
### 鏍稿績鍔熻兘
- **瀹炴椂鏁版嵁鐩戞帶**锛氭帴鏀跺苟鏄剧ずGPU銆丆PU銆佸唴瀛樹娇鐢ㄧ巼
- **浠〃鐩樺睍绀?*锛氳繘搴︽潯寮忔寚鏍囧崱鐗囷紝鐩磋鏄剧ず鍚勯」鏁版嵁
- **鍘嗗彶鍥捐〃**锛氫娇鐢╢l_chart缁樺埗鎶樼嚎鍥撅紝灞曠ず鍘嗗彶瓒嬪娍
- **鏍峰紡鍒囨崲**锛?濂楅璁句富棰橈紙Dark銆乀ech Blue銆丆yberpunk銆丮inimal Light锛?- **鍝嶅簲寮忚璁?*锛氶€傞厤涓嶅悓Android璁惧灞忓箷灏哄

### 涓嶱C鐗堜竴鑷存€?- **鏁版嵁缁撴瀯**锛氬畬鍏ㄥ吋瀹筆C鐗堢殑SystemData鍜孏puData鏍煎紡
- **缃戠粶鍗忚**锛氫娇鐢ㄧ浉鍚岀殑UDP骞挎挱鍗忚锛堢鍙?876锛?- **UI甯冨眬**锛氫繚鎸佷笌PC鐗堢浉鍚岀殑瑙嗚椋庢牸鍜屼氦浜掗€昏緫
- **涓婚绯荤粺**锛?濂椾富棰橀鑹查厤缃笌PC鐗堝畬鍏ㄤ竴鑷?
## 鎶€鏈爤

- **妗嗘灦**锛欶lutter 3.x
- **璇█**锛欴art
- **鐘舵€佺鐞?*锛歅rovider
- **鍥捐〃搴?*锛歠l_chart
- **缃戠粶閫氫俊**锛歞art:io (RawDatagramSocket)

## 椤圭洰缁撴瀯

```
gpu_monitor_android/
鈹溾攢鈹€ lib/
鈹?  鈹溾攢鈹€ main.dart                    # 搴旂敤鍏ュ彛
鈹?  鈹溾攢鈹€ models/                      # 鏁版嵁妯″瀷
鈹?  鈹?  鈹溾攢鈹€ gpu_data.dart           # GPU鏁版嵁缁撴瀯
鈹?  鈹?  鈹溾攢鈹€ system_data.dart        # 绯荤粺鏁版嵁缁撴瀯
鈹?  鈹?  鈹斺攢鈹€ monitor_message.dart    # 缃戠粶娑堟伅鏍煎紡
鈹?  鈹溾攢鈹€ services/                    # 鏈嶅姟灞?鈹?  鈹?  鈹斺攢鈹€ network_service.dart    # UDP缃戠粶鏈嶅姟
鈹?  鈹溾攢鈹€ providers/                   # 鐘舵€佺鐞?鈹?  鈹?  鈹斺攢鈹€ monitor_provider.dart   # 鐩戞帶鏁版嵁鎻愪緵鑰?鈹?  鈹溾攢鈹€ screens/                     # 鐣岄潰
鈹?  鈹?  鈹溾攢鈹€ connection_screen.dart  # 杩炴帴鐣岄潰
鈹?  鈹?  鈹溾攢鈹€ monitor_screen.dart     # 鐩戞帶鐣岄潰
鈹?  鈹?  鈹斺攢鈹€ error_screen.dart       # 閿欒鐣岄潰
鈹?  鈹溾攢鈹€ widgets/                     # 缁勪欢
鈹?  鈹?  鈹溾攢鈹€ gauge_card.dart         # 浠〃鐩樺崱鐗?鈹?  鈹?  鈹溾攢鈹€ gpu_dashboard.dart      # GPU浠〃鐩?鈹?  鈹?  鈹溾攢鈹€ system_dashboard.dart   # 绯荤粺浠〃鐩?鈹?  鈹?  鈹斺攢鈹€ history_chart.dart      # 鍘嗗彶鍥捐〃
鈹?  鈹斺攢鈹€ themes/                      # 涓婚
鈹?      鈹斺攢鈹€ app_themes.dart         # 4濂楅璁句富棰?鈹溾攢鈹€ android/                         # Android閰嶇疆
鈹?  鈹溾攢鈹€ app/
鈹?  鈹?  鈹斺攢鈹€ src/
鈹?  鈹?      鈹斺攢鈹€ main/
鈹?  鈹?          鈹溾攢鈹€ AndroidManifest.xml
鈹?  鈹?          鈹溾攢鈹€ kotlin/
鈹?  鈹?          鈹斺攢鈹€ res/
鈹?  鈹溾攢鈹€ build.gradle
鈹?  鈹斺攢鈹€ settings.gradle
鈹溾攢鈹€ pubspec.yaml                     # Flutter渚濊禆閰嶇疆
鈹斺攢鈹€ analysis_options.yaml            # 浠ｇ爜鍒嗘瀽閰嶇疆
```

## 鐜瑕佹眰

### 寮€鍙戠幆澧?- Flutter SDK 3.x 鎴栨洿楂樼増鏈?- Dart SDK 3.0 鎴栨洿楂樼増鏈?- Android Studio 鎴?VS Code
- Android SDK (API level 21+)

### Android璁惧瑕佹眰
- Android 5.0 (API level 21) 鎴栨洿楂樼増鏈?- 鏀寔UDP缃戠粶閫氫俊
- 寤鸿浣跨敤Android 8.0+浠ヨ幏寰楁渶浣充綋楠?
## 鏋勫缓姝ラ

### 1. 瀹夎Flutter SDK

```bash
# 涓嬭浇Flutter SDK
# https://flutter.dev/docs/get-started/install

# 楠岃瘉瀹夎
flutter doctor
```

### 2. 閰嶇疆Android寮€鍙戠幆澧?
```bash
# 瀹夎Android Studio
# 閰嶇疆Android SDK
# 璁剧疆鐜鍙橀噺ANDROID_HOME
```

### 3. 鑾峰彇渚濊禆

```bash
cd gpu_monitor_android
flutter pub get
```

### 4. 杩愯璋冭瘯鐗堟湰

```bash
# 杩炴帴Android璁惧鎴栧惎鍔ㄦā鎷熷櫒
flutter run
```

### 5. 鏋勫缓鍙戝竷鐗堟湰APK

```bash
# 鏋勫缓鏈鍚岮PK
flutter build apk --release

# APK浣嶇疆锛歜uild/app/outputs/flutter-apk/app-release.apk
```

### 6. 鐢熸垚绛惧悕APK

#### 鏂规硶涓€锛氫娇鐢ˋndroid Studio
1. 鎵撳紑椤圭洰鍦ˋndroid Studio涓?2. 閫夋嫨 Build 鈫?Generate Signed Bundle / APK
3. 閫夋嫨 APK
4. 鍒涘缓鎴栭€夋嫨瀵嗛挜搴?5. 閫夋嫨 release 鏋勫缓绫诲瀷
6. 鐐瑰嚮 Finish

#### 鏂规硶浜岋細浣跨敤鍛戒护琛?```bash
# 1. 鐢熸垚瀵嗛挜搴?keytool -genkey -v -keystore ~/pc-monitor-key.jks -keyalg RSA -keysize 2048 -validity 10000 -alias pc-monitor

# 2. 鍒涘缓key.properties鏂囦欢
# 鍦╝ndroid鐩綍涓嬪垱寤簁ey.properties锛?# storePassword=<your-password>
# keyPassword=<your-password>
# keyAlias=pc-monitor
# storeFile=<path-to-your-jks-file>

# 3. 閰嶇疆build.gradle
# 鍦╝ndroid/app/build.gradle涓坊鍔犵鍚嶉厤缃?
# 4. 鏋勫缓绛惧悕APK
flutter build apk --release
```

## 浣跨敤璇存槑

### 1. 鍚姩妫€娴嬬锛圥C锛?```bash
# 鍦≒C涓婅繍琛屾娴嬬绋嬪簭
./collector.exe
```

### 2. 鍚姩鏄剧ず绔紙Android锛?1. 瀹夎APK鍒癆ndroid璁惧
2. 鎵撳紑搴旂敤
3. 鐐瑰嚮"Connect"鎸夐挳寮€濮嬬洃鍚琔DP骞挎挱
4. 绛夊緟鎺ユ敹妫€娴嬬鏁版嵁

### 3. 鍔熻兘鎿嶄綔
- **鏍峰紡鍒囨崲**锛氱偣鍑婚《閮?Style: Dark"鎸夐挳鍒囨崲涓婚
- **鏂紑杩炴帴**锛氱偣鍑?Disconnect"鎸夐挳鍋滄鎺ユ敹鏁版嵁
- **閲嶆柊杩炴帴**锛氬湪閿欒鐣岄潰鐐瑰嚮"Retry"鎸夐挳

## 缃戠粶閰嶇疆

### 闃茬伀澧欒缃?纭繚PC绔槻鐏鍏佽UDP绔彛9876鐨勫嚭绔欓€氫俊锛?```bash
# Windows
netsh advfirewall firewall add rule name="PC Monitor" dir=out action=allow protocol=UDP localport=9876

# Linux
sudo ufw allow out 9876/udp
```

### 缃戠粶瑕佹眰
- PC鍜孉ndroid璁惧蹇呴』鍦ㄥ悓涓€灞€鍩熺綉鍐?- 鏀寔UDP骞挎挱閫氫俊
- 寤鸿浣跨敤Wi-Fi杩炴帴浠ヨ幏寰楁渶浣虫€ц兘

## 鏁呴殰鎺掗櫎

### 1. 鏃犳硶鎺ユ敹鏁版嵁
- 妫€鏌C鍜孉ndroid璁惧鏄惁鍦ㄥ悓涓€缃戠粶
- 楠岃瘉闃茬伀澧欒缃?- 纭妫€娴嬬绋嬪簭姝ｅ湪杩愯

### 2. 搴旂敤宕╂簝
- 妫€鏌ndroid鐗堟湰鏄惁绗﹀悎瑕佹眰
- 鏌ョ湅logcat鏃ュ織鑾峰彇璇︾粏閿欒淇℃伅
- 纭繚鏈夎冻澶熺殑瀛樺偍绌洪棿

### 3. 鎬ц兘闂
- 鍑忓皯鍘嗗彶鏁版嵁闀垮害锛堜慨鏀筸axHistoryLength锛?- 闄嶄綆鏁版嵁鏇存柊棰戠巼
- 鍏抽棴涓嶅繀瑕佺殑鍚庡彴搴旂敤

## 寮€鍙戣鏄?
### 娣诲姞鏂板姛鑳?1. 鍦╩odels鐩綍娣诲姞鏁版嵁妯″瀷
2. 鍦╯ervices鐩綍瀹炵幇涓氬姟閫昏緫
3. 鍦╬roviders鐩綍娣诲姞鐘舵€佺鐞?4. 鍦╯creens鍜寃idgets鐩綍瀹炵幇UI

### 鑷畾涔変富棰?淇敼`lib/themes/app_themes.dart`涓殑棰滆壊閰嶇疆锛?```dart
const AppTheme(
  name: 'Custom',
  bgColor: Color(0xFFYourColor),
  // ... 鍏朵粬棰滆壊閰嶇疆
);
```

## 鐗堟湰鍘嗗彶

- **v1.0.0** (2026-06-12)
  - 鍒濆鐗堟湰鍙戝竷
  - 瀹炵幇涓嶱C鐗堜竴鑷寸殑鏍稿績鍔熻兘
  - 鏀寔4濂椾富棰樻牱寮?  - 鍝嶅簲寮忓竷灞€璁捐

## 璁稿彲璇?
鏈」鐩熀浜嶱C鐗圙PU鐩戞帶搴旂敤寮€鍙戯紝閬靛惊鐩稿悓鐨勮鍙瘉鍗忚銆?
## 鑱旂郴鏂瑰紡

濡傛湁闂鎴栧缓璁紝璇疯仈绯诲紑鍙戝洟闃熴€?