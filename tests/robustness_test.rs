//! Regression coverage for docs/BACKLOG.md story 4.2: the analyzer and the
//! robots.txt parser must never panic on arbitrary input, however garbled.
//! Uses a small deterministic xorshift PRNG (no `rand` dependency, no
//! external entropy) so the run is reproducible.

use politebot_core::{analyze, parse_robots};

struct XorShift32(u32);

impl XorShift32 {
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        self.0
    }
}

fn random_bytes(seed: u32, len: usize) -> Vec<u8> {
    let mut rng = XorShift32(seed | 1);
    (0..len).map(|_| (rng.next() % 256) as u8).collect()
}

#[test]
fn analyzer_never_panics_on_random_byte_sequences() {
    for seed in 1..200u32 {
        let bytes = random_bytes(seed, 64);
        let text = String::from_utf8_lossy(&bytes).to_string();
        let _ = analyze(&text);
    }
}

#[test]
fn robots_parser_never_panics_on_random_byte_sequences() {
    for seed in 1..200u32 {
        let bytes = random_bytes(seed, 64);
        let text = String::from_utf8_lossy(&bytes).to_string();
        let rules = parse_robots(&text);
        let _ = rules.is_allowed("AnyBot", "/some/path");
    }
}
