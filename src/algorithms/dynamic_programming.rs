pub fn fibonacci_tab(n: usize) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut table = vec![0u64; n + 1];
    table[1] = 1;
    for i in 2..=n {
        table[i] = table[i - 1] + table[i - 2];
    }
    table[n]
}

pub fn knapsack_01(weights: &[usize], values: &[usize], capacity: usize) -> usize {
    let n = weights.len();
    let mut dp = vec![vec![0usize; capacity + 1]; n + 1];

    for i in 1..=n {
        for c in 0..=capacity {
            dp[i][c] = dp[i - 1][c];
            let w = weights[i - 1];
            if w <= c {
                let take = values[i - 1] + dp[i - 1][c - w];
                dp[i][c] = dp[i][c].max(take);
            }
        }
    }

    dp[n][capacity]
}

pub fn lcs_length(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut dp = vec![vec![0usize; b.len() + 1]; a.len() + 1];

    for i in 1..=a.len() {
        for j in 1..=b.len() {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    dp[a.len()][b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dp_algorithms() {
        assert_eq!(fibonacci_tab(10), 55);
        assert_eq!(knapsack_01(&[1, 3, 4], &[15, 20, 30], 4), 35);
        assert_eq!(lcs_length("ALGORUST", "RUSTACEAN"), 4);
    }
}
