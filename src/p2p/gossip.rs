use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct GossipRound {
    pub round: usize,
    pub informed_nodes: Vec<usize>,
}

pub fn simulate_gossip(graph: &[Vec<usize>], seed: usize, rounds: usize) -> Vec<GossipRound> {
    let mut informed = BTreeSet::new();
    informed.insert(seed);
    let mut history = Vec::with_capacity(rounds + 1);

    history.push(GossipRound {
        round: 0,
        informed_nodes: informed.iter().copied().collect(),
    });

    for r in 1..=rounds {
        let current: Vec<usize> = informed.iter().copied().collect();
        for node in current {
            for &peer in &graph[node] {
                informed.insert(peer);
            }
        }

        history.push(GossipRound {
            round: r,
            informed_nodes: informed.iter().copied().collect(),
        });
    }

    history
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gossip_spreads() {
        let graph = vec![vec![1], vec![2], vec![3], vec![]];
        let rounds = simulate_gossip(&graph, 0, 3);
        assert_eq!(
            rounds.last().map(|g| g.informed_nodes.clone()),
            Some(vec![0, 1, 2, 3])
        );
    }
}
