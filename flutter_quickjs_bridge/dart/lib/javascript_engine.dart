import 'dart:convert';
import 'dart:ffi';
import 'dart:typed_data';

import 'package:ffi/ffi.dart';

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2025-03-22 12:56:33
 * @modify date 2025-03-22 12:56:33
 * @desc [description]
 */

typedef JavaScriptEngineNewRust = Pointer<Void> Function();
typedef JavaScriptEngineNewDart = Pointer<Void> Function();

typedef JavaScriptEngineFreeRust = Void Function(Pointer<Void>);
typedef JavaScriptEngineFreeDart = void Function(Pointer<Void>);

typedef JavaScriptEngineEvalRust = Void Function(Pointer<Void>, Pointer<Uint8>, Uint32);
typedef JavaScriptEngineEvalDart = void Function(Pointer<Void>, Pointer<Uint8>, int);

final class NativeString {

  static T fromDartString<T>(String text, T Function(Pointer<Uint8> pointer, int length) action) {
    final bytes = utf8.encode(text);
    final heapPointer = malloc.allocate<Uint8>(bytes.length);
    final heapBytes = heapPointer.asTypedList(bytes.length);
    heapBytes.setAll(0, bytes);
    final result = action(heapPointer, bytes.length);
    malloc.free(heapPointer);
    return result;
  }
}

class JavaScriptEngine {
  static DynamicLibrary? library;
  static JavaScriptEngineNewDart? _javascript_engine_new;
  static JavaScriptEngineFreeDart? _javascript_engine_free;
  static JavaScriptEngineEvalDart? _javascript_engine_eval;

  late Pointer<Void> _pointer;
  
  static void initializeDynamicLibrary() {
    library ??= DynamicLibrary.open("/media/dream-lab/Development/Project/meshel_ai_terminal/flutter_quickjs_bridge/rust/target/release/libflutter_quickjs_bridge.so");
    _javascript_engine_new = library!.lookupFunction<JavaScriptEngineNewRust, JavaScriptEngineNewDart>("javascript_engine_new");
    _javascript_engine_free = library!.lookupFunction<JavaScriptEngineFreeRust, JavaScriptEngineFreeDart>("javascript_engine_free");
    _javascript_engine_eval = library!.lookupFunction<JavaScriptEngineEvalRust, JavaScriptEngineEvalDart>("javascript_engine_eval");
  }

  JavaScriptEngine() {
    if (library == null) {
      initializeDynamicLibrary();
    }
    _pointer = _javascript_engine_new!();
  }

  void eval(String sourceCode) {
    NativeString.fromDartString(sourceCode, (pointer, length) {
      _javascript_engine_eval!(_pointer, pointer, length);
    });
  }

  void dispose() {
    _javascript_engine_free!(_pointer);
  }
}

