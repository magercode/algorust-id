#[derive(Debug, Clone)]
pub struct PeerStat {
    pub id: String,
    pub latency_ms: u32,
    pub reliability: f32,
    pub bandwidth_mbps: u32,
}

pub fn rank_peers(mut peers: Vec<PeerStat>) -> Vec<PeerStat> {
    peers.sort_by(|a, b| {
        let score_a = peer_score(a);
        let score_b = peer_score(b);
        score_b.total_cmp(&score_a)
    });
    peers
}

fn peer_score(peer: &PeerStat) -> f32 {
    let latency_component = 1.0 / (peer.latency_ms.max(1) as f32);
    latency_component * 0.25 + peer.reliability * 0.45 + (peer.bandwidth_mbps as f32) * 0.30
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer_rank() {
        let peers = vec![
            PeerStat {
                id: "A".to_string(),
                latency_ms: 20,
                reliability: 0.98,
                bandwidth_mbps: 100,
            },
            PeerStat {
                id: "B".to_string(),
                latency_ms: 10,
                reliability: 0.75,
                bandwidth_mbps: 30,
            },
        ];
        let ranked = rank_peers(peers);
        assert_eq!(ranked[0].id, "A");
    }
}
