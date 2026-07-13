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
