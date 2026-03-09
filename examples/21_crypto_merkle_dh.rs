use algorust::crypto::diffie_hellman::{public_key, shared_secret};
use algorust::crypto::merkle::merkle_root;

fn main() {
    let transactions = vec![
        "Alice->Bob:10".to_string(),
        "Bob->Charlie:4".to_string(),
        "Dave->Alice:2".to_string(),
        "Charlie->Eve:1".to_string(),
    ];
    let root = merkle_root(&transactions);
    println!("Merkle root: {root:?}");

    let p = 23u128;
    let g = 5u128;
    let alice_private = 6u128;
    let bob_private = 15u128;
    let alice_public = public_key(g, alice_private, p);
    let bob_public = public_key(g, bob_private, p);
    let alice_shared = shared_secret(bob_public, alice_private, p);
    let bob_shared = shared_secret(alice_public, bob_private, p);

    println!("Alice public key: {alice_public}");
    println!("Bob public key  : {bob_public}");
    println!("Shared secret A : {alice_shared}");
    println!("Shared secret B : {bob_shared}");
}
