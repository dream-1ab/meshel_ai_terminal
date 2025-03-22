


import 'package:dart_quickjs_bridge/javascript_engine.dart';

void main() {
  final engine = JavaScriptEngine();
  engine.registerDartFunctionAsModule("dart/exports", [
    ("hello", (action, bytes, tag) {
      print("Called from js engine.");
      return 0;
    })
  ]);

  engine.eval("""
import {sleep} from "core/threading";
// import {hello} from "dart/exports";
let x = 0
while (true) {
  x += 1;
  console.log(x)
  sleep(16)
  // hello(1, new Uint8Array([1, 2, 3]).buffer, 2)
}
""");
  engine.dispose();
}
