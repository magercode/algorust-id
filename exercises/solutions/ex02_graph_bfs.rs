use std::collections::VecDeque;

fn bfs_shortest_hops(graph: &[Vec<usize>], start: usize) -> Vec<Option<usize>> {
    let mut dist = vec![None; graph.len()];
    let mut q = VecDeque::new();
    dist[start] = Some(0);
    q.push_back(start);

    while let Some(node) = q.pop_front() {
        let current = dist[node].unwrap_or(0);
        for &next in &graph[node] {
            if dist[next].is_none() {
                dist[next] = Some(current + 1);
                q.push_back(next);
            }
        }
    }

    dist
}

fn main() {
    let g = vec![vec![1, 2], vec![3], vec![3, 4], vec![5], vec![5], vec![]];
    println!("{:?}", bfs_shortest_hops(&g, 0));
}
