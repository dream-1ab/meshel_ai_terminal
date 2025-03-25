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
typedef JavaScriptEngineNewRust = Pointer<Void> Function(Uint64 engine_id);
typedef JavaScriptEngineNewDart = Pointer<Void> Function(int engineId);

// javascript_engine_free
typedef JavaScriptEngineFreeRust = Void Function(Pointer<Void> engine_ptr);
typedef JavaScriptEngineFreeDart = void Function(Pointer<Void> enginePtr);

//javascript_engine_eval
typedef JavaScriptEngineEvalRust = Void Function(Pointer<Void> engine_ptr, Pointer<NativeString> script_ptr);
typedef JavaScriptEngineEvalDart = void Function(Pointer<Void> enginePtr, Pointer<NativeString> scriptPtr);

// javascript_engine_register_dart_function
typedef JavaScriptEngineRegisterDartFunctionRust = Void Function(Pointer<Void> engine_ptr, Pointer<Uint8> function_name_ptr, Uint64 function_name_length);
typedef JavaScriptEngineRegisterDartFunctionDart = void Function(Pointer<Void> enginePtr, Pointer<Uint8> functionPtr, int functionNameLength);

// javascript_engine_set_dart_callback_functions
typedef JavaScriptEngineSetDartCallbackFunctionRust = Void Function(Pointer<Void> engine_ptr, Uint64 dart_function_call, Uint64 javascript_function_register);
typedef JavaScriptEngineSetDartCallbackFunctionDart = void Function(Pointer<Void> enginePtr, int dartFunctionCall, int javascriptFunctionRegister);


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

typedef JavaScriptFunctionFreeRust = Void Function(Pointer<_JavaScriptFunction>);
typedef JavaScriptFunctionFreeDart = void Function(Pointer<_JavaScriptFunction>);

typedef JavaScriptFunctionCallRust = Int32 Function(Pointer<_JavaScriptFunction> ptr, Int32 action, Pointer<Uint8> bytes, Uint32 length, Int32 tag);
typedef JavaScriptFunctionCallDart = int Function(Pointer<_JavaScriptFunction> ptr, int action, Pointer<Uint8> bytes, int length, int tag);

final class _JavaScriptFunction extends Struct {
  external Pointer<Void> function;
}

class JavaScriptFunction {
  static JavaScriptFunctionFreeDart? _javascript_free;
  static JavaScriptFunctionCallDart? _javascript_call;

  bool _disposed = false;
  final Pointer<_JavaScriptFunction> _pointer;

  JavaScriptFunction._fromPointer(Pointer<_JavaScriptFunction> pointer) : _pointer = pointer {
    _disposed = false;
  }

  void dispose() {
    assert(!_disposed);
    if (_disposed) throw Exception("Twise free of javascript function pointer.");
    _javascript_free!(_pointer);
    _disposed = true;
  }

  static void _initializeNativeLibrary() {
    _javascript_free ??= JavaScriptEngine.library!.lookupFunction<JavaScriptFunctionFreeRust, JavaScriptFunctionFreeDart>("javascript_function_free");
    _javascript_call ??= JavaScriptEngine.library!.lookupFunction<JavaScriptFunctionCallRust, JavaScriptFunctionCallDart>("javascript_function_call");
  }

  int call(int action, int memorySizeOfArguments, Function(Uint8List argumentMemory) argumentFiller, int tag, ) {
    final memory_ptr = malloc.allocate<Uint8>(memorySizeOfArguments);
    final bytes = memory_ptr.asTypedList(memorySizeOfArguments);
    argumentFiller(bytes);
    final result = this.call_with_buffer(action, memory_ptr, bytes.length, tag);
    malloc.free(memory_ptr);
    return result;
  }

  int call_with_buffer(int action, Pointer<Uint8> memory_ptr, int memory_length, int tag) {
    return _javascript_call!(_pointer, action, memory_ptr, memory_length, tag);
  }
}

typedef CallDartFunctionCallback = int Function(int action, Uint8List bytes, int tag);
typedef RegisterJavaScriptFunctionCallback = int Function(String name, JavaScriptFunction function);

abstract class JavaScriptEngine {
  static DynamicLibrary? library;
  static JavaScriptEngineNewDart? _javascript_engine_new;
  static JavaScriptEngineFreeDart? _javascript_engine_free;
  static JavaScriptEngineEvalDart? _javascript_engine_eval;
  static JavaScriptEngineRegisterDartFunctionDart? _javascript_engine_register_dart_function;
  static JavaScriptEngineSetDartCallbackFunctionDart? _javascript_engine_set_dart_callback_functions;

  late Pointer<Void> _pointer;
  late int _engineId_;
  int get engineId => _engineId_;
  bool _disposed = false;
  final List<JavaScriptFunction> _registeredJavaScriptFunctions = [];
  
  static void initializeDynamicLibrary() {
    const errorCode = -2;
    _javascriptCallHandlerPointer = Pointer.fromFunction<Int32 Function(Uint64 engine_id, Int32 action, Pointer<Uint8> bytes_pointer, Uint32 length, Uint64 id, Int32 tag)>(_callFromJavaScriptHandler, errorCode).address;
    _javascriptFunctionRegisterHandlerPointer = Pointer.fromFunction<Int32 Function(Uint64 engine_id, Pointer<Uint8> function_name_ptr, Uint32 function_name_length, Pointer<_JavaScriptFunction> javascript_function)>(_javascriptFunctionRegisterHandler, errorCode).address;

    library ??= DynamicLibrary.open("../rust/target/release/libflutter_quickjs_bridge.so");
    _javascript_engine_new = library!.lookupFunction<JavaScriptEngineNewRust, JavaScriptEngineNewDart>("javascript_engine_new");
    _javascript_engine_free = library!.lookupFunction<JavaScriptEngineFreeRust, JavaScriptEngineFreeDart>("javascript_engine_free");
    _javascript_engine_eval = library!.lookupFunction<JavaScriptEngineEvalRust, JavaScriptEngineEvalDart>("javascript_engine_eval");
    _javascript_engine_register_dart_function = library!.lookupFunction<JavaScriptEngineRegisterDartFunctionRust, JavaScriptEngineRegisterDartFunctionDart>("javascript_engine_register_dart_function");
    _javascript_engine_set_dart_callback_functions = library!.lookupFunction<JavaScriptEngineSetDartCallbackFunctionRust, JavaScriptEngineSetDartCallbackFunctionDart>("javascript_engine_set_dart_callback_functions");

    JavaScriptFunction._initializeNativeLibrary();
  }

  JavaScriptEngine() {
    if (library == null) {
      initializeDynamicLibrary();
    }
    this._engineId_ = JavaScriptEngine._lastAllocatedEngineId;
    JavaScriptEngine._lastAllocatedEngineId++;
    _pointer = _javascript_engine_new!(this.engineId);
    _javascript_engine_set_dart_callback_functions!(_pointer, _javascriptCallHandlerPointer!, _javascriptFunctionRegisterHandlerPointer!);
    this.initialize();
    _javascriptEngines[this.engineId] = this;
    this._disposed = false;
  }

  void initialize();

  void eval(String sourceCode) {
    NativeString.fromDartString(sourceCode, (nativeString) {
      _javascript_engine_eval!(_pointer, nativeString);
    });
  }

  void registerDartFunctionAsModule(String moduleName, List<(String, CallDartFunctionCallback)> functions) {
    final moduleBytes = DartModuleObjectBuilder(
      name: moduleName,
      functions: functions.map((function) => DartFunctionObjectBuilder(
        name: function.$1,
        functionId: _registerJavaScriptCallHandler(this.engineId, function.$2)
      )).toList()
    ).toBytes();
    // File("dart_module.bin").writeAsBytesSync(moduleBytes, flush: true);
    {
      final pointer = malloc.allocate<Uint8>(moduleBytes.length);
      final heapBuffer = pointer.asTypedList(moduleBytes.length);
      heapBuffer.setAll(0, moduleBytes);
      _javascript_engine_register_dart_function!(this._pointer, pointer, heapBuffer.length);
      malloc.free(pointer);
    }
  }

  int onJavaScriptFunctionRegistered(String name, JavaScriptFunction function);

  void dispose() {
    assert(!_disposed);
    if (_disposed) throw Exception("Twise free of javascript engine pointer.");
    
    final registeredCallCallbacks = JavaScriptEngine._javascriptCallCallbacks.keys.where((key) => key >= this.engineId * 1000000 && key < ((this.engineId + 1) * 1000000)).toList();
    for (final callbackId in registeredCallCallbacks) {
      JavaScriptEngine._javascriptCallCallbacks.remove(callbackId);
    }
    _javascriptEngines.remove(this.engineId);
    this._registeredJavaScriptFunctions.forEach((function) {
      if (!function._disposed) function.dispose();
    });
    this._registeredJavaScriptFunctions.clear();
    _disposed = true;
    _javascript_engine_free!(_pointer);
  }

  static Map<int, CallDartFunctionCallback> _javascriptCallCallbacks = {};
  static Map<int, JavaScriptEngine> _javascriptEngines = {};
  static int _lastAllocatedEngineId = 1;
  static int _lastAllocatedId = 1;
  static int? _javascriptCallHandlerPointer;
  static int? _javascriptFunctionRegisterHandlerPointer;

  static int _registerJavaScriptCallHandler(int engineId, CallDartFunctionCallback callback) {
    final allocatedId = (engineId * 1000000) + ++_lastAllocatedId;
    _javascriptCallCallbacks[allocatedId] = callback;
    return allocatedId;
  }

  static int _callFromJavaScriptHandler(int engine_id, int action, Pointer<Uint8> bytesPointer, int length, int callback_id, int tag) {
    final id = callback_id;
    final callback = _javascriptCallCallbacks[id];
    if (callback == null) return -1;
    return callback(action, bytesPointer.asTypedList(length), tag);
  }

  static int _javascriptFunctionRegisterHandler(int engine_id, Pointer<Uint8> name_ptr, int name_length, Pointer<_JavaScriptFunction> javascript_function) {
    final functionName = utf8.decode(name_ptr.asTypedList(name_length));
    final engine = _javascriptEngines[engine_id];
    if (engine == null) return -1;
    final javascriptFunction = JavaScriptFunction._fromPointer(javascript_function);
    engine._registeredJavaScriptFunctions.add(javascriptFunction);
    final result = engine.onJavaScriptFunctionRegistered(functionName, javascriptFunction);
    return result;
  }
  
}
