/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2025-03-22 12:52:03
 * @modify date 2025-03-22 12:52:03
 * @desc [description]
*/
import { sleep } from "core/threading";
let counter = 0;
while (true) {
    console.log({ name: "hello world", count: counter++ });
    sleep(32);
}
