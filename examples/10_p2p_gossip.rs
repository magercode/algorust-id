use algorust::p2p::gossip::simulate_gossip;
use algorust::p2p::peer_selection::{PeerStat, rank_peers};

fn main() {
    let overlay = vec![vec![1, 2], vec![3], vec![3, 4], vec![5], vec![5], vec![]];
    let rounds = simulate_gossip(&overlay, 0, 4);
    for snapshot in rounds {
        println!(
            "Round {} -> informed nodes {:?}",
            snapshot.round, snapshot.informed_nodes
        );
    }

    let peers = vec![
        PeerStat {
            id: "peer-jakarta".to_string(),
            latency_ms: 18,
            reliability: 0.96,
            bandwidth_mbps: 120,
        },
        PeerStat {
            id: "peer-bandung".to_string(),
            latency_ms: 12,
            reliability: 0.87,
            bandwidth_mbps: 70,
        },
        PeerStat {
            id: "peer-surabaya".to_string(),
            latency_ms: 25,
            reliability: 0.99,
            bandwidth_mbps: 150,
        },
    ];

    println!();
    println!("Ranking peer terbaik:");
    for (idx, peer) in rank_peers(peers).iter().enumerate() {
        println!(
            "{}. {} (latency={}ms, reliability={}, bw={}Mbps)",
            idx + 1,
            peer.id,
            peer.latency_ms,
            peer.reliability,
            peer.bandwidth_mbps
        );
    }
}
