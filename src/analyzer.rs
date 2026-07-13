//! Static analysis of pasted scraper source. Fleshed out in a follow-up commit.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Green,
}

#[derive(Debug, Clone)]
pub struct PolitenessScore {
    pub verdict: Verdict,
}

pub fn analyze(_source: &str) -> PolitenessScore {
    PolitenessScore {
        verdict: Verdict::Green,
    }
}
