# ALGORUST

`algorust` adalah proyek pembelajaran algoritma modern dengan Rust, mencakup:
- algoritma fundamental (sorting, searching, graph, dynamic programming),
- algoritma sistem P2P (gossip dissemination dan strategi pemilihan peer),
- dasar kriptografi (cipher klasik, key exchange, dan Merkle tree).

Seluruh materi ditulis dalam bahasa Indonesia, berorientasi praktik, dan siap dipakai untuk:
- belajar mandiri,
- kelas internal tim engineering,
- bootcamp/komunitas.

## 1. Tujuan Pembelajaran

Setelah menyelesaikan materi ini, peserta diharapkan mampu:
- memahami kompleksitas waktu dan ruang untuk algoritma umum,
- mengimplementasikan algoritma dengan pendekatan idiomatik Rust,
- menjelaskan konsep jaringan P2P dan trade-off desainnya,
- menerapkan konsep kriptografi dasar untuk integritas dan pertukaran kunci,
- menyelesaikan exercise terstruktur dari tingkat dasar sampai menengah.

## 2. Struktur Proyek

```text
algorust/
├── src/
│   ├── algorithms/
│   │   ├── sorting.rs
│   │   ├── searching.rs
│   │   ├── graph.rs
│   │   └── dynamic_programming.rs
│   ├── p2p/
│   │   ├── gossip.rs
│   │   └── peer_selection.rs
│   ├── crypto/
│   │   ├── classical.rs
│   │   ├── diffie_hellman.rs
│   │   └── merkle.rs
│   ├── lib.rs
│   └── main.rs
├── examples/
├── exercises/
│   ├── README.md
│   ├── starter/
│   └── solutions/
└── docs/
```

## 3. Cara Menjalankan

Prasyarat:
- Rust stable (disarankan lewat `rustup`).

Perintah utama:

```bash
cargo check
cargo test
```

Menjalankan contoh:

```bash
cargo run --example 01_sorting_searching
cargo run --example 02_graph_dp
cargo run --example 10_p2p_gossip
cargo run --example 20_crypto_basics
cargo run --example 21_crypto_merkle_dh
```

## 4. Silabus 12 Minggu (Rekomendasi)

1. Minggu 1: Dasar Rust (ownership, borrowing, enum, pattern matching).
2. Minggu 2: Kompleksitas algoritma, Big-O, benchmarking sederhana.
3. Minggu 3: Sorting (Bubble, Merge, Quick) dan analisis trade-off.
4. Minggu 4: Searching (Linear, Binary), precondition data terurut.
5. Minggu 5: Graph traversal (BFS), shortest path berbobot (Dijkstra).
6. Minggu 6: Dynamic Programming (Fibonacci tabulation, Knapsack, LCS).
7. Minggu 7: Konsep jaringan P2P dan desain overlay.
8. Minggu 8: Gossip protocol dan simulasi penyebaran informasi.
9. Minggu 9: Strategi pemilihan peer berbasis metrik jaringan.
10. Minggu 10: Kriptografi dasar (Caesar, XOR stream) untuk pemahaman konsep.
11. Minggu 11: Diffie-Hellman dan konsep pertukaran kunci.
12. Minggu 12: Merkle tree, integritas data, mini capstone.

## 5. Peta Modul

### A. Algoritma Fundamental
- `sorting.rs`: `bubble_sort`, `merge_sort`, `quick_sort`
- `searching.rs`: `linear_search`, `binary_search`
- `graph.rs`: `bfs_shortest_hops`, `dijkstra`
- `dynamic_programming.rs`: `fibonacci_tab`, `knapsack_01`, `lcs_length`

### B. Algoritma Sistem P2P
- `gossip.rs`: simulasi propagasi informasi per ronde.
- `peer_selection.rs`: ranking peer berdasarkan latency, reliability, bandwidth.

### C. Kriptografi (Edukasi)
- `classical.rs`: Caesar cipher, XOR stream cipher sederhana.
- `diffie_hellman.rs`: modular exponentiation dan shared secret.
- `merkle.rs`: konstruksi root hash untuk integritas data.

## 6. Standar Pembelajaran Profesional

- Selalu validasi konsep dengan test (`cargo test`) sebelum benchmarking.
- Pisahkan kode edukasi dan kode produksi; modul crypto pada repo ini berfokus pada konsep.
- Dokumentasikan asumsi input/output saat membuat algoritma baru.
- Gunakan review checklist: correctness, complexity, edge case, readability.
- Beri prioritas pada keamanan memori dan error handling yang eksplisit.

## 7. Catatan Keamanan Penting

Modul kriptografi di proyek ini dibuat untuk pembelajaran konsep algoritmik.
Untuk kebutuhan produksi:
- gunakan library kriptografi terstandar dan teraudit,
- ikuti rekomendasi parameter keamanan terkini,
- lakukan security review independen.

## 8. Roadmap Pengembangan Lanjutan

Prioritas lanjutan yang disarankan:
1. Menambah benchmark (`criterion`) untuk membandingkan kompleksitas praktis.
2. Menambah simulasi churn node di P2P (node join/leave).
3. Menambah contoh autentikasi message (MAC/signature).
4. Menambah CI pipeline (fmt, clippy, test, doc test).

## 9. Kontribusi

Kontribusi sangat terbuka:
1. Fork dan buat branch fitur.
2. Tambahkan test untuk setiap perubahan algoritma.
3. Pastikan `cargo test` lulus.
4. Buat PR dengan deskripsi teknis singkat dan terukur.
