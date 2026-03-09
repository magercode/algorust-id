use std::collections::BTreeSet;

#[derive(Debug)]
struct Round {
    round: usize,
    informed: Vec<usize>,
}

fn simulate_gossip(graph: &[Vec<usize>], seed: usize, rounds: usize) -> Vec<Round> {
    let mut known = BTreeSet::new();
    let mut history = Vec::new();
    known.insert(seed);
    history.push(Round {
        round: 0,
        informed: known.iter().copied().collect(),
    });

    for r in 1..=rounds {
        let now: Vec<usize> = known.iter().copied().collect();
        for node in now {
            for &peer in &graph[node] {
                known.insert(peer);
            }
        }
        history.push(Round {
            round: r,
            informed: known.iter().copied().collect(),
        });
    }

    history
}

fn main() {
    let overlay = vec![vec![1, 2], vec![3], vec![3, 4], vec![5], vec![5], vec![]];
    println!("{:#?}", simulate_gossip(&overlay, 0, 3));
}
