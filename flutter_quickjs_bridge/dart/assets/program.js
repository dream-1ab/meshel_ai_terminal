
import {log, error, info, warn, print} from "core/console"
import {sleep} from "core/threading";
import {exportJavaScriptFunction} from "dart/interop"

import {call_my_dart_function} from "dart/exports";

let x = 0
let buffer = new Uint8Array(1024 * 128).buffer

function myJsFunction(action, byte_buffer, tag) {
    let bytes = new Uint8Array(byte_buffer)
    // console.log("Called from dart", action, bytes.length, tag)
    // log(bytes)
    // console.log("Jello")
    const result = call_my_dart_function(1, buffer, 2);
    return x++ + result
}
exportJavaScriptFunction("myJsFunction", myJsFunction)

// for (let i = 0; i < 100; i++) {
//     // console.log(x)
//     //   sleep(16)
//     call_my_dart_function(i, buffer, i + 2)
// }

