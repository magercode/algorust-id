pub fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1u128;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result.saturating_mul(base) % modulus;
        }
        exp >>= 1;
        base = base.saturating_mul(base) % modulus;
    }
    result
}

pub fn public_key(generator: u128, private_key: u128, prime_modulus: u128) -> u128 {
    mod_pow(generator, private_key, prime_modulus)
}

pub fn shared_secret(peer_public_key: u128, private_key: u128, prime_modulus: u128) -> u128 {
    mod_pow(peer_public_key, private_key, prime_modulus)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diffie_hellman_key_exchange() {
        let p = 23u128;
        let g = 5u128;

        let alice_private = 6u128;
        let bob_private = 15u128;

        let alice_public = public_key(g, alice_private, p);
        let bob_public = public_key(g, bob_private, p);

        let alice_shared = shared_secret(bob_public, alice_private, p);
        let bob_shared = shared_secret(alice_public, bob_private, p);
        assert_eq!(alice_shared, bob_shared);
    }
}
