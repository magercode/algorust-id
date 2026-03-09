use algorust::algorithms::dynamic_programming::{fibonacci_tab, knapsack_01, lcs_length};
use algorust::algorithms::graph::{WeightedGraph, bfs_shortest_hops, dijkstra};

fn main() {
    let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
    let hops = bfs_shortest_hops(&graph, 0);
    println!("BFS shortest hops dari node 0: {hops:?}");

    let weighted: WeightedGraph = vec![
        vec![(1, 4), (2, 1)],
        vec![(3, 1)],
        vec![(1, 2), (3, 5)],
        vec![],
    ];
    println!("Dijkstra dari node 0: {:?}", dijkstra(&weighted, 0));

    println!("Fibonacci(20) = {}", fibonacci_tab(20));
    println!(
        "Knapsack = {}",
        knapsack_01(&[2, 3, 4, 5], &[3, 4, 5, 8], 8)
    );
    println!(
        "LCS('ALGORUST', 'RUSTACEAN') = {}",
        lcs_length("ALGORUST", "RUSTACEAN")
    );
}
