// Exercise 02: Hitung jarak hop terpendek dari node start menggunakan BFS.
// Graph:
// 0 -> [1, 2]
// 1 -> [3]
// 2 -> [3, 4]
// 3 -> [5]
// 4 -> [5]
// 5 -> []

fn bfs_shortest_hops(_graph: &[Vec<usize>], _start: usize) -> Vec<Option<usize>> {
    // TODO: implementasi BFS dengan queue
    vec![]
}

fn main() {
    let g = vec![vec![1, 2], vec![3], vec![3, 4], vec![5], vec![5], vec![]];
    let hops = bfs_shortest_hops(&g, 0);
    println!("hops dari 0: {hops:?}");
}
