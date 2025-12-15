#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result = Vec::new();
    
    for &value in values {
        if value == 0 {
            result.push(0x00);
            continue;
        }
        
        let mut num = value;
        let mut bytes = Vec::new();
        
        loop {
            let byte = (num & 0x7F) as u8;
            num >>= 7;
            
            if bytes.is_empty() {
                bytes.push(byte);
            } else {
                bytes.push(byte | 0x80);
            }
            
            if num == 0 {
                break;
            }
        }
        
        bytes.reverse();
        result.extend_from_slice(&bytes);
    }
    
    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut result = Vec::new();
    let mut i = 0;
    
    while i < bytes.len() {
        let mut value: u32 = 0;
        let mut has_more = true;
        
        while has_more {
            if i >= bytes.len() {
                return Err(Error::IncompleteNumber);
            }
            
            let byte = bytes[i];
            i += 1;
            
            value = (value << 7) | ((byte & 0x7F) as u32);
            has_more = (byte & 0x80) != 0;
        }
        
        result.push(value);
    }
    
    Ok(result)
}
