use algorust::crypto::classical::{caesar_decrypt, caesar_encrypt, xor_stream};

fn main() {
    let plain = "Belajar Rust itu menyenangkan";
    let encrypted = caesar_encrypt(plain, 5);
    let decrypted = caesar_decrypt(&encrypted, 5);
    println!("Caesar plain    : {plain}");
    println!("Caesar encrypted: {encrypted}");
    println!("Caesar decrypted: {decrypted}");

    let key = b"algorust";
    let secret = b"pesan rahasia p2p";
    let cipher = xor_stream(secret, key);
    let restored = xor_stream(&cipher, key);
    println!();
    println!("XOR cipher bytes: {cipher:?}");
    println!("XOR restored    : {}", String::from_utf8_lossy(&restored));
}
