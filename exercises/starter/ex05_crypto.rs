// Exercise 05:
// 1) buat fungsi caesar_decrypt
// 2) buat XOR roundtrip
// 3) hitung shared secret DH sederhana

fn caesar_encrypt(_input: &str, _shift: u8) -> String {
    // TODO
    String::new()
}

fn caesar_decrypt(_input: &str, _shift: u8) -> String {
    // TODO
    String::new()
}

fn xor_stream(_data: &[u8], _key: &[u8]) -> Vec<u8> {
    // TODO
    vec![]
}

fn mod_pow(_base: u128, _exp: u128, _modulus: u128) -> u128 {
    // TODO
    0
}

fn main() {
    let cipher = caesar_encrypt("ALGORUST", 3);
    println!("cipher: {cipher}");
    println!("plain : {}", caesar_decrypt(&cipher, 3));

    let x = xor_stream(b"hello", b"key");
    println!("xor bytes: {x:?}");

    let p = 23u128;
    let g = 5u128;
    let a = 6u128;
    let b = 15u128;
    let a_pub = mod_pow(g, a, p);
    let b_pub = mod_pow(g, b, p);
    println!("shared A: {}", mod_pow(b_pub, a, p));
    println!("shared B: {}", mod_pow(a_pub, b, p));
}
