use algorust::{PROJECT_NAME, PROJECT_TAGLINE};

fn main() {
    println!("=== {} ===", PROJECT_NAME.to_uppercase());
    println!("{PROJECT_TAGLINE}");
    println!();
    println!("Mulai belajar dengan contoh siap jalan:");
    println!("  cargo run --example 01_sorting_searching");
    println!("  cargo run --example 02_graph_dp");
    println!("  cargo run --example 10_p2p_gossip");
    println!("  cargo run --example 20_crypto_basics");
    println!("  cargo run --example 21_crypto_merkle_dh");
    println!();
    println!("Lanjutkan dengan latihan di folder exercises/.");
}
