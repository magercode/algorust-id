// Exercise 04: Simulasi gossip protocol.
// Keluaran: daftar node yang sudah menerima pesan di setiap ronde.

#[derive(Debug)]
struct Round {
    round: usize,
    informed: Vec<usize>,
}

fn simulate_gossip(_graph: &[Vec<usize>], _seed: usize, _rounds: usize) -> Vec<Round> {
    // TODO: gunakan struktur set agar node tidak dobel
    vec![]
}

fn main() {
    let overlay = vec![vec![1, 2], vec![3], vec![3, 4], vec![5], vec![5], vec![]];
    let rounds = simulate_gossip(&overlay, 0, 3);
    println!("{rounds:#?}");
}
