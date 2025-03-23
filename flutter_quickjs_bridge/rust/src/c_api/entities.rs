/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2025-03-22 22:08:33
 * @modify date 2025-03-22 22:08:33
 * @desc [description]
*/

pub struct EncodedString<'a> {
    bytes: &'a [u8]
}

impl<'a> EncodedString<'a> {
    pub fn new_from_bytes(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub fn as_str(&self) -> &'a str {
        let length = u32::from_le_bytes([self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3]]) as usize;
        std::str::from_utf8(&self.bytes[4..length]).expect("Cannot decode utf8 string from byte array")
    }
}
