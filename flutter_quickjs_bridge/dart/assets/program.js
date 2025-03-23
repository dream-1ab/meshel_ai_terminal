
import {log, error, info, warn, print} from "core/console"
import {sleep} from "core/threading";

import {call_my_dart_function} from "dart/exports";

let x = 0
let buffer = new Uint8Array(1024 * 128).buffer

for (let i = 0; i < 10000; i++) {
    //   console.log(x)
    x += 1;
    //   sleep(16)
    call_my_dart_function(1, buffer, 2)
}
