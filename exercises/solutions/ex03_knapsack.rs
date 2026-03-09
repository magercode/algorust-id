fn knapsack_01(weights: &[usize], values: &[usize], capacity: usize) -> usize {
    let n = weights.len();
    let mut dp = vec![vec![0usize; capacity + 1]; n + 1];

    for i in 1..=n {
        for c in 0..=capacity {
            dp[i][c] = dp[i - 1][c];
            if weights[i - 1] <= c {
                let take = values[i - 1] + dp[i - 1][c - weights[i - 1]];
                dp[i][c] = dp[i][c].max(take);
            }
        }
    }

    dp[n][capacity]
}

fn main() {
    println!("{}", knapsack_01(&[2, 3, 4, 5], &[3, 4, 5, 8], 8));
}
