fn shift_char(ch: char, shift: i16) -> char {
    if !ch.is_ascii_alphabetic() {
        return ch;
    }
    let base = if ch.is_ascii_uppercase() { b'A' } else { b'a' } as i16;
    let pos = ch as i16 - base;
    (base + (pos + shift).rem_euclid(26)) as u8 as char
}

fn caesar_encrypt(input: &str, shift: u8) -> String {
    input.chars().map(|ch| shift_char(ch, shift as i16)).collect()
}

fn caesar_decrypt(input: &str, shift: u8) -> String {
    input
        .chars()
        .map(|ch| shift_char(ch, -(shift as i16)))
        .collect()
}

fn xor_stream(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, b)| b ^ key[i % key.len()])
        .collect()
}

fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    let mut result = 1u128;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus;
    }
    result
}

fn main() {
    let c = caesar_encrypt("ALGORUST", 3);
    println!("{c}");
    println!("{}", caesar_decrypt(&c, 3));

    let x = xor_stream(b"hello", b"key");
    let p = xor_stream(&x, b"key");
    println!("{x:?}");
    println!("{:?}", String::from_utf8_lossy(&p));

    let p_mod = 23u128;
    let g = 5u128;
    let a = 6u128;
    let b = 15u128;
    let a_pub = mod_pow(g, a, p_mod);
    let b_pub = mod_pow(g, b, p_mod);
    println!("{}", mod_pow(b_pub, a, p_mod));
    println!("{}", mod_pow(a_pub, b, p_mod));
}
