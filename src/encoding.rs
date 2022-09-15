// Binary-to-text encoding in base 2048

const CHARSET: &str = include_str!("./charset.txt");
const BITS_PER_CHAR: usize = 11;
const BITS_PER_BYTE: usize = 8;


pub fn encode(data: &[u8]) -> String {
    let mut result = String::new();
    let mut buffer = 0;
    let mut bits_in_buffer = 0;

    for byte in data {
        buffer |= (*byte as u32) << bits_in_buffer;
        bits_in_buffer += BITS_PER_BYTE;

        while bits_in_buffer >= BITS_PER_CHAR {
            let index = (buffer & 0x7FF) as usize;
            result.push(CHARSET.chars().nth(index).unwrap());
            buffer >>= BITS_PER_CHAR;
            bits_in_buffer -= BITS_PER_CHAR;
        }
    }

    if bits_in_buffer > 0 {
        let index = (buffer & 0x7FF) as usize;
        result.push(CHARSET.chars().nth(index).unwrap());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = "0";
        let s = "Hello, world!".as_bytes();
        assert_eq!(encode(s), x);
        Ok(())
    }
}