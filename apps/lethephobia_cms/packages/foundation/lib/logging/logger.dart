import 'package:logging/logging.dart';

void configureLogger(Level logLevel) {
  Logger.root.level = logLevel;
  Logger.root.onRecord.listen((record) {
    var msg = '${record.level.name}: ${record.time}: ${record.message}';
    if (record.error != null) {
      msg += ' ${record.error}';
    }
    if (record.stackTrace != null) {
      msg += '\n${record.stackTrace}';
    }
    print(msg);
  });
}
