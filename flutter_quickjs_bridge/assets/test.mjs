import {sleep} from "core/threading"

let index = 0
while (true) {
    // let bytes = new Uint8Array(1024)
    console.log("hello world!", index)
    index++
    sleep(16)
}
