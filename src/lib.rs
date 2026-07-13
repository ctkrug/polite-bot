//! politebot-core: the analysis engine behind polite_bot.
//!
//! Compiled to WebAssembly so the paste-and-score flow runs entirely in the
//! browser — nothing pasted here ever touches a server.

mod analyzer;
mod robots;

use wasm_bindgen::prelude::*;

pub use analyzer::{analyze, PolitenessScore, Verdict};
pub use robots::{parse as parse_robots, RobotsRules};

/// Returns the crate version, mostly so the site can prove the wasm module loaded.
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Scores pasted scraper source for politeness issues. Returns a JSON string
/// (`{"verdict": "...", "findings": [...]}`) — the wasm boundary stays
/// primitive-only so no serde/JS-value glue is needed on either side.
#[wasm_bindgen]
pub fn score_scraper(source: &str) -> String {
    analyze(source).to_json()
}

/// True if `path` is allowed for `agent` under the given robots.txt text.
#[wasm_bindgen]
pub fn check_robots(robots_txt: &str, agent: &str, path: &str) -> bool {
    parse_robots(robots_txt).is_allowed(agent, path)
}
