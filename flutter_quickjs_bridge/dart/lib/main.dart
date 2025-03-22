


import 'package:dart_quickjs_bridge/javascript_engine.dart';

void main() {
  final engine = JavaScriptEngine();
  engine.eval("""
import {sleep} from "core/threading";

let x = 0
while (true) {
  x += 1;
  console.log(x)
  sleep(16)
}
""");
  engine.dispose();
}
