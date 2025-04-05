use hex;

use base64;

pub fn bytes_to_base64(bytes: &[u8]) -> String {
    base64::encode(bytes)
}

pub fn base64_create_file(b64_string: String, filename: &str) -> Result<(), std::io::Error> {
    std::fs::write(filename, b64_string)
}
