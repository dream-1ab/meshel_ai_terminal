/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2025-03-22 03:53:26
 * @modify date 2025-03-22 03:53:26
 * @desc [description]
 */

import {log, info, warn, error, print} from "core/console"
import {sleep} from "core/threading"
import {setDartCallHandler, callDart} from "dart/interop"

let counter = 0

setDartCallHandler((age, name) => {
    print("From Rust: ", name, age)
})

while (true) {
    print({name: "hello world", count: counter++})
    sleep(32)
}
