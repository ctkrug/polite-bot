# polite_bot

**▶ Live demo — [apps.charliekrug.com/polite-bot](https://apps.charliekrug.com/polite-bot/)**

[![CI](https://github.com/ctkrug/polite-bot/actions/workflows/ci.yml/badge.svg)](https://github.com/ctkrug/polite-bot/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-35e08a.svg)](LICENSE)

Score your scraper before a site bans you.

polite_bot reads a web scraper's source and flags the things that get scrapers
IP-banned or make headlines: a missing rate limit, no `User-Agent`
identification, and requests that ignore the target site's `robots.txt`. Paste a
Python `requests` loop (or a JavaScript `fetch`) and you get a red/yellow/green
verdict in about a second, with the exact line to fix. It runs entirely in your
browser: a Rust engine compiled to WebAssembly does the analysis, so nothing you
paste ever leaves your machine.

## Who it's for

Developers writing early-stage or one-off scraping scripts (research data pulls,
personal projects, internal tooling) who want a fast sanity check before they
hit run. Think of it as a linter you run before a commit, not an enterprise
compliance tool.

## Why it's different

Most "robots.txt checker" tools only look at the *target site*. They tell you
what a site allows, not whether *your code* is well-behaved. polite_bot scores
the scraper itself against the practices that keep scrapers polite: identify
yourself, throttle your requests, and respect the rules the site publishes.

## What it catches

- **Missing or default User-Agent.** No `User-Agent` header, or a library
  default like `python-requests/2.x`, gets flagged as a distinct finding, with a
  one-click diff that inserts a real identifier for Python `requests` and JS
  `fetch` calls.
- **No rate limiting.** Detects the absence of `time.sleep`, `asyncio.sleep`,
  `setTimeout`, and backoff libraries (`tenacity`, `@retry`-style decorators),
  and points at the request loop that has none.
- **robots.txt violations.** A from-scratch parser (not a regex) that
  understands `User-agent` grouping, `Allow`/`Disallow` longest-match
  precedence, and `Crawl-delay`. Paste a target path and its robots.txt to
  cross-check live.
- **Line-accurate findings.** Every finding names a 1-based line and renders as
  a colored gutter marker in the editor, not just a disconnected list.
- **Shareable reports.** A "copy report" button produces a markdown summary of
  the verdict for a PR description or code review comment.

## Sample output

Paste this worst-case loop:

```python
import requests

for url in urls:
    requests.get(url)
```

polite_bot returns:

```
verdict: red
  L4  no User-Agent header found — scrapers should identify themselves   [fix]
  L4  no rate limiting or backoff detected between requests
```

Click **fix** on the first finding and it hands back:

```python
import requests

headers = {"User-Agent": "polite-bot/1.0 (+https://github.com/ctkrug/polite-bot)"}
for url in urls:
    requests.get(url, headers=headers)
```

## How it works

- **Engine:** Rust, compiled to `wasm32-unknown-unknown` via `wasm-bindgen`.
  Pure static analysis plus a from-scratch robots.txt parser. No network calls,
  no server. The wasm boundary stays primitive-only (strings in, JSON strings
  out) so there is no serde dependency.
- **Site:** a static, dependency-free HTML/CSS/JS front end that loads the wasm
  module directly. No build tooling to run it, no backend to host.

See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for a map of the codebase.

## Local development

```sh
# run the Rust test suite
cargo test

# build the wasm module + bindings into site/pkg/
scripts/build-wasm.sh

# serve site/ (app.js is an ES module, which browsers block from a bare
# file:// URL, so use a local server rather than opening the file directly)
python3 -m http.server 8000 --directory site
# then visit http://localhost:8000
```

See [`docs/VISION.md`](docs/VISION.md) for the product vision,
[`docs/BACKLOG.md`](docs/BACKLOG.md) for the build plan, and
[`docs/DESIGN.md`](docs/DESIGN.md) for the visual direction.

## License

MIT. See [`LICENSE`](LICENSE).

---

More of Charlie's projects → [apps.charliekrug.com](https://apps.charliekrug.com)
