/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2025-03-22 12:52:03
 * @modify date 2025-03-22 12:52:03
 * @desc [description]
*/

import {sleep} from "core/threading"

let counter = 0
for (let a = 0; a < 100; a++) {
    console.log({name: "hello world", count: counter++})
    sleep(16)
}
