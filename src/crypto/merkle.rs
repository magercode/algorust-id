use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash_str(value: &str) -> String {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

pub fn merkle_root(leaves: &[String]) -> Option<String> {
    if leaves.is_empty() {
        return None;
    }

    let mut layer: Vec<String> = leaves.iter().map(|leaf| hash_str(leaf)).collect();

    while layer.len() > 1 {
        let mut next = Vec::with_capacity(layer.len().div_ceil(2));
        for pair in layer.chunks(2) {
            if pair.len() == 2 {
                next.push(hash_str(&(pair[0].clone() + &pair[1])));
            } else {
                next.push(hash_str(&(pair[0].clone() + &pair[0])));
            }
        }
        layer = next;
    }

    layer.into_iter().next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_not_empty() {
        let tx = vec![
            "TX-1".to_string(),
            "TX-2".to_string(),
            "TX-3".to_string(),
            "TX-4".to_string(),
        ];
        let root = merkle_root(&tx);
        assert!(root.is_some());
    }
}
