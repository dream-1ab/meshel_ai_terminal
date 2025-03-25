/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2025-03-22 12:48:22
 * @modify date 2025-03-22 12:48:22
 * @desc [description]
 */
import * as _console from "core/console";
import { utf8_decode, utf8_encode } from "core/rust";
globalThis.console = _console;
class TextEncoderDecoder {
    encode(text) {
        return utf8_encode(text);
    }
    decode(bytes) {
        return utf8_decode(bytes);
    }
}
globalThis.TextEncoder = TextEncoderDecoder;
globalThis.TextDecoder = TextEncoderDecoder;
