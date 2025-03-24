


import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';

import 'package:dart_quickjs_bridge/javascript_engine.dart';
import 'package:ffi/ffi.dart';


class MyJsEngine extends JavaScriptEngine {
  JavaScriptFunction? _myJsFunction;
  @override
  int onJavaScriptFunctionRegistered(String name, JavaScriptFunction function) {
    // print("onJavaScriptFunctionRegistered: $name");
    if (name == "myJsFunction") {
      _myJsFunction = function;
    }
    return 0;
  }
  
  @override
  void initialize() {
    this.registerDartFunctionAsModule("dart/exports", [
      ("call_my_dart_function", myDartFunction),
    ]);

    this.registerDartFunctionAsModule("dart/io", [
      ("open_file", (int action, Uint8List bytes, int tag) {
        return 0;
      }),
    ]);
  }

  int myDartFunction(int action, Uint8List bytes, int tag) {
    // print("myDartFunction: $action, ${bytes.length}, $tag");
    // final payload = utf8.encode("Hello world");
    // return _myJsFunction!.call(action, payload.length, (memory) => memory.setAll(0, payload), 99);
    return 2;
  }
}

Future<void> main() async {
  final engine = MyJsEngine();

  final source = await File("./assets/program.js").readAsString();
  final watch = Stopwatch();
  engine.eval(source);
  {
    final ptr = malloc.allocate<Uint8>(1024 * 128);
    watch.start();
    var result = 0;
    for (int a = 0; a < 100; a++) {
      result = engine._myJsFunction!.call_with_buffer(0, ptr, 1024 * 128, 99);
    }
    watch.stop();
    malloc.free(ptr);
    print("Evaluation time: ${watch.elapsedMicroseconds} micros\n\n\n\n");
    print("Result: $result");
  }

  engine.dispose();
}
