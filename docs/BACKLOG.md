# Backlog

Epics and stories for the build. Every story lists concrete, verifiable
acceptance criteria — no vibes. Stories are marked `[ ]` until built.

## Epic 1 — Instant politeness verdict (the wow moment)

The first epic is the demo. Nothing else gets built before this works.

- [x] **1.1 (wow) — Paste code, get a red/yellow/green verdict with the exact
  flagged line.**
  - Pasting a Python `requests` loop with no `User-Agent` and no sleep call
    yields a **Red** verdict with findings pointing at the request line.
  - Pasting code with both a `User-Agent` header and a `time.sleep`/
    `asyncio.sleep`/`setTimeout`-style call yields **Green** with zero
    findings.
  - The verdict updates within 300ms of pasting, computed entirely
    client-side via the wasm module (no network round trip).

- [x] **1.2 — One-click "add recommended headers" diff.**
  - Clicking "Fix" on a missing-`User-Agent` finding produces a unified diff
    snippet inserting a `User-Agent` header at the request call site.
  - A "Copy fixed code" button copies the patched snippet to the clipboard.

- [x] **1.3 — robots.txt cross-check.**
  - Pasting or fetching a robots.txt and a target path flags scraped paths
    that are `Disallow`ed for the matching `User-agent` group.
  - An empty, missing, or malformed robots.txt is treated as allow-all —
    never a crash or an unhandled error state.

## Epic 2 — Static analysis depth

- [ ] **2.1 — Recognize more rate-limiting patterns.**
  - Recognizes `time.sleep`, `asyncio.sleep`, and `setTimeout`-based
    throttling as satisfying the rate-limit check (already implemented for
    the v0 heuristic; this story extends it to at least one retry/backoff
    library convention, e.g. `tenacity` or exponential-backoff libraries).
  - Source that doesn't match any recognized language/framework pattern
    degrades to a **Yellow** verdict with an explanatory finding, rather
    than a false Red.

- [ ] **2.2 — Detect generic/default User-Agent strings.**
  - Flags default library User-Agent strings (e.g. `python-requests/2.x`,
    Node's default `node-fetch`) as a distinct finding from "missing
    entirely," since a default string is still not real identification.

- [ ] **2.3 — Line-accurate findings rendered in the editor.**
  - Every finding carries a 1-based line number pointing at the offending
    call (implemented in `politebot-core`; this story wires it into the UI).
  - Findings render as colored gutter markers in the paste editor, not just
    a disconnected list.

- [ ] **2.4 — Design polish: gutter markers.**
  - Gutter markers use `docs/DESIGN.md`'s verdict palette (`--warn` /
    `--danger`) rather than a generic red dot, and fade in with the 90ms
    reveal transition instead of appearing instantly.

## Epic 3 — Trust & transparency

- [ ] **3.1 — Explain each rule.**
  - Every finding links to a one-paragraph explanation of why it matters
    (IP-ban risk, ToS violation, basic scraping courtesy).

- [ ] **3.2 — Shareable report.**
  - A "Copy report" action produces a markdown summary (verdict + findings)
    suitable for pasting into a PR description or code review comment.

- [ ] **3.3 — Sample snippet library.**
  - A "Try an example" control loads at least 3 pre-built snippets (polite,
    rate-limited-only, worst-case) so a first-time visitor sees the tool
    work with zero typing.

- [ ] **3.4 — Design polish: explanations and report styling.**
  - Rule explanations and the copied markdown report use the same
    monospace type pairing and terminal-mono voice as the rest of the
    page — no generic prose-styled help-center look.

## Epic 4 — Ship polish

- [ ] **4.1 — Design polish pass.**
  - The page matches `docs/DESIGN.md`'s tokens and direction at 390px,
    768px, and 1440px widths, with no unstyled native controls.

- [ ] **4.2 — Wasm performance & reliability.**
  - The wasm module loads and scores a 200-line snippet in under 50ms in a
    current Chrome or Firefox.
  - The analyzer never panics on arbitrary UTF-8 input (covered by a
    property-style regression test alongside the existing unit tests).

- [ ] **4.3 — Landing page & docs.**
  - `site/` opens directly into the paste-and-score workspace (per
    `docs/DESIGN.md` layout intent) with a one-line tagline explaining the
    wow moment — no separate marketing page to click through first.
  - README documents local dev (`cargo test`, `scripts/build-wasm.sh`) and
    links the live site once deployed.
