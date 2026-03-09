use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

pub type WeightedGraph = Vec<Vec<(usize, u32)>>;

pub fn bfs_shortest_hops(graph: &[Vec<usize>], start: usize) -> Vec<Option<usize>> {
    let mut dist = vec![None; graph.len()];
    let mut q = VecDeque::new();
    dist[start] = Some(0);
    q.push_back(start);

    while let Some(node) = q.pop_front() {
        let base = dist[node].unwrap_or(0);
        for &next in &graph[node] {
            if dist[next].is_none() {
                dist[next] = Some(base + 1);
                q.push_back(next);
            }
        }
    }

    dist
}

pub fn dijkstra(graph: &WeightedGraph, start: usize) -> Vec<u32> {
    let mut dist = vec![u32::MAX; graph.len()];
    let mut heap = BinaryHeap::new();
    dist[start] = 0;
    heap.push((Reverse(0u32), start));

    while let Some((Reverse(cost), node)) = heap.pop() {
        if cost > dist[node] {
            continue;
        }

        for &(next, weight) in &graph[node] {
            let candidate = cost.saturating_add(weight);
            if candidate < dist[next] {
                dist[next] = candidate;
                heap.push((Reverse(candidate), next));
            }
        }
    }

    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_algorithms() {
        let unweighted = vec![vec![1, 2], vec![3], vec![3], vec![]];
        let hops = bfs_shortest_hops(&unweighted, 0);
        assert_eq!(hops[3], Some(2));

        let weighted: WeightedGraph = vec![
            vec![(1, 4), (2, 1)],
            vec![(3, 1)],
            vec![(1, 2), (3, 5)],
            vec![],
        ];
        let dist = dijkstra(&weighted, 0);
        assert_eq!(dist[3], 4);
    }
}
