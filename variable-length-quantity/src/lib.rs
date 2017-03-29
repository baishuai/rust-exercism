/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    for &v in values.iter() {
        let start = bytes.len();
        let mut v = v;
        bytes.insert(start, ((v as u8) & 0x7F));
        v >>= 7;
        while v > 0 {
            bytes.insert(start, (v as u8 & 0x7F) | 0x80);
            v >>= 7;
        }
    }
    bytes
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
    if bytes[bytes.len() - 1] > ((1 << 7) - 1) {
        return Err("No complete u32 numbers found");
    }
    let mut vals = Vec::new();
    let mut sum: u32 = 0;
    for &byte in bytes {
        if sum > (1 << 25) {
            return Err("Number overflows a u32");
        }
        sum <<= 7;
        if byte > ((1 << 7) - 1) {
            sum += (byte & 0x7F) as u32;
        } else {
            vals.push(sum + byte as u32);
            sum = 0;
        }
    }
    Ok(vals)
}
