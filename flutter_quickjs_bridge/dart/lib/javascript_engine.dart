import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';

import 'package:dart_quickjs_bridge/fbs/generated/javascript_engine_wrapper_javascript_engine_dart_wrapper_generated.dart';
import 'package:ffi/ffi.dart';

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2025-03-22 12:56:33
 * @modify date 2025-03-22 12:56:33
 * @desc [description]
 */

// javascript_engine_new
typedef JavaScriptEngineNewRust = Pointer<Void> Function();
typedef JavaScriptEngineNewDart = Pointer<Void> Function();

// javascript_engine_free
typedef JavaScriptEngineFreeRust = Void Function(Pointer<Void>);
typedef JavaScriptEngineFreeDart = void Function(Pointer<Void>);

//javascript_engine_eval
typedef JavaScriptEngineEvalRust = Void Function(Pointer<Void>, Pointer<NativeString>,);
typedef JavaScriptEngineEvalDart = void Function(Pointer<Void>, Pointer<NativeString>,);

// javascript_engine_register_dart_function
typedef JavaScriptEngineRegisterDartFunctionRust = Void Function(Pointer<Void>, Pointer<Uint8>, Uint64);
typedef JavaScriptEngineRegisterDartFunctionDart = void Function(Pointer<Void>, Pointer<Uint8>, int);

// javascript_engine_set_dart_callback_function
typedef JavaScriptEngineSetDartCallbackFunctionRust = Void Function(Pointer<Void>, Uint64);
typedef JavaScriptEngineSetDartCallbackFunctionDart = void Function(Pointer<Void>, int);


final class NativeString extends Struct {
  external Pointer<Uint8> ptr;
  @Uint32()
  external int length;

  static T fromDartString<T>(String text, T Function(Pointer<NativeString> string) action) {
    final bytes = utf8.encode(text);
    final heapPointer = malloc.allocate<Uint8>(bytes.length);
    final heapBytes = heapPointer.asTypedList(bytes.length);
    heapBytes.setAll(0, bytes);
    final nativeStringPointer = malloc.allocate<NativeString>(sizeOf<NativeString>());
    nativeStringPointer[0].ptr = heapPointer;
    nativeStringPointer[0].length = bytes.length;
    final result = action(nativeStringPointer);
    malloc.free(heapPointer);
    malloc.free(nativeStringPointer);
    return result;
  }

  String toDartString() {
    return utf8.decode(ptr.asTypedList(length));
  }
}

typedef DartFunctionCallback = int Function(int action, Uint8List bytes, int tag);

class JavaScriptEngine {
  static DynamicLibrary? library;
  static JavaScriptEngineNewDart? _javascript_engine_new;
  static JavaScriptEngineFreeDart? _javascript_engine_free;
  static JavaScriptEngineEvalDart? _javascript_engine_eval;
  static JavaScriptEngineRegisterDartFunctionDart? _javascript_engine_register_dart_function;
  static JavaScriptEngineSetDartCallbackFunctionDart? _javascript_engine_set_dart_callback_function;
  late Pointer<Void> _pointer;
  
  static void initializeDynamicLibrary() {
    const errorCode = -2;
    _javascriptCallHandlerPointer = Pointer.fromFunction<Int32 Function(Int32 action, Pointer<Uint8> bytes_pointer, Uint32 length, Uint64 id, Int32 tag)>(_javaScriptCallHandler, errorCode).address;

    library ??= DynamicLibrary.open("/media/dream-lab/Development/Project/meshel_ai_terminal/flutter_quickjs_bridge/rust/target/debug/libflutter_quickjs_bridge.so");
    _javascript_engine_new = library!.lookupFunction<JavaScriptEngineNewRust, JavaScriptEngineNewDart>("javascript_engine_new");
    _javascript_engine_free = library!.lookupFunction<JavaScriptEngineFreeRust, JavaScriptEngineFreeDart>("javascript_engine_free");
    _javascript_engine_eval = library!.lookupFunction<JavaScriptEngineEvalRust, JavaScriptEngineEvalDart>("javascript_engine_eval");
    _javascript_engine_register_dart_function = library!.lookupFunction<JavaScriptEngineRegisterDartFunctionRust, JavaScriptEngineRegisterDartFunctionDart>("javascript_engine_register_dart_function");
    _javascript_engine_set_dart_callback_function = library!.lookupFunction<JavaScriptEngineSetDartCallbackFunctionRust, JavaScriptEngineSetDartCallbackFunctionDart>("javascript_engine_set_dart_callback_function");
  }

  JavaScriptEngine() {
    if (library == null) {
      initializeDynamicLibrary();
    }
    _pointer = _javascript_engine_new!();
    _javascript_engine_set_dart_callback_function!(_pointer, _javascriptCallHandlerPointer!);
  }

  void eval(String sourceCode) {
    NativeString.fromDartString(sourceCode, (nativeString) {
      _javascript_engine_eval!(_pointer, nativeString);
    });
  }

  void registerDartFunctionAsModule(String moduleName, List<(String, DartFunctionCallback)> functions) {
    const errorCode = -2;
    final moduleBytes = DartModuleObjectBuilder(
      name: moduleName,
      functions: functions.map((function) => DartFunctionObjectBuilder(
        name: function.$1,
        functionId: _registerJavaScriptCallHandler(function.$2)
      )).toList()
    ).toBytes();
    File("dart_module.bin").writeAsBytesSync(moduleBytes, flush: true);
    {
      final pointer = malloc.allocate<Uint8>(moduleBytes.length);
      final heapBuffer = pointer.asTypedList(moduleBytes.length);
      heapBuffer.setAll(0, moduleBytes);
      _javascript_engine_register_dart_function!(this._pointer, pointer, heapBuffer.length);
      malloc.free(pointer);
    }
  }

  void dispose() {
    _javascript_engine_free!(_pointer);
  }

  static Map<int, DartFunctionCallback> _callbacks = {};
  static int _id = 0;
  static int? _javascriptCallHandlerPointer;

  static int _registerJavaScriptCallHandler(DartFunctionCallback callback) {
    final allocatedId = ++_id;
    _callbacks[allocatedId] = callback;
    return allocatedId;
  }
  static int _javaScriptCallHandler(int action, Pointer<Uint8> bytesPointer, int length, int id, int tag) {
    final callback = _callbacks[id];
    if (callback == null) return -1;
    return callback(action, bytesPointer.asTypedList(length), tag);
  }
}
