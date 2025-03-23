


import 'dart:io';

import 'package:dart_quickjs_bridge/javascript_engine.dart';

Future<void> main() async {
  final engine = JavaScriptEngine();
  engine.registerDartFunctionAsModule("dart/exports", [
    ("call_my_dart_function", (action, bytes, tag) {
      // print("Called from js engine.");
      return 10;
    })
  ]);

  final source = await File("./assets/program.js").readAsString();
  final watch = Stopwatch();
  watch.start();
  engine.eval(source);
  watch.stop();
  print("Evaluation time: ${watch.elapsedMicroseconds} micros");
  engine.dispose();
}
