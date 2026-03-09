pub fn linear_search(data: &[i32], target: i32) -> Option<usize> {
    data.iter().position(|v| *v == target)
}

pub fn binary_search(data: &[i32], target: i32) -> Option<usize> {
    let mut low = 0usize;
    let mut high = data.len();

    while low < high {
        let mid = low + (high - low) / 2;
        match data[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let values = vec![1, 3, 5, 7, 9, 11];
        assert_eq!(linear_search(&values, 7), Some(3));
        assert_eq!(binary_search(&values, 7), Some(3));
        assert_eq!(binary_search(&values, 2), None);
    }
}
