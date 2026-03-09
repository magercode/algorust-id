pub fn caesar_encrypt(input: &str, shift: u8) -> String {
    input
        .chars()
        .map(|ch| shift_char(ch, shift as i16))
        .collect()
}

pub fn caesar_decrypt(input: &str, shift: u8) -> String {
    input
        .chars()
        .map(|ch| shift_char(ch, -(shift as i16)))
        .collect()
}

fn shift_char(ch: char, shift: i16) -> char {
    if !ch.is_ascii_alphabetic() {
        return ch;
    }

    let base = if ch.is_ascii_uppercase() { b'A' } else { b'a' } as i16;
    let normalized = ch as i16 - base;
    let shifted = (normalized + shift).rem_euclid(26);
    (base + shifted) as u8 as char
}

pub fn xor_stream(data: &[u8], key: &[u8]) -> Vec<u8> {
    if key.is_empty() {
        return data.to_vec();
    }
    data.iter()
        .enumerate()
        .map(|(i, b)| b ^ key[i % key.len()])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caesar_roundtrip() {
        let plain = "Halo Rust";
        let encrypted = caesar_encrypt(plain, 3);
        let decrypted = caesar_decrypt(&encrypted, 3);
        assert_eq!(decrypted, plain);
    }

    #[test]
    fn test_xor_roundtrip() {
        let msg = b"algorust";
        let key = b"key";
        let c = xor_stream(msg, key);
        let p = xor_stream(&c, key);
        assert_eq!(p, msg);
    }
}
