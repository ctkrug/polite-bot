# polite_bot

[![CI](https://github.com/ctkrug/polite-bot/actions/workflows/ci.yml/badge.svg)](https://github.com/ctkrug/polite-bot/actions/workflows/ci.yml)

**Paste your scraping script. Get a politeness score.**

polite_bot is a static analyzer for web scrapers. Paste a Python `requests` loop
(or similar), and it flags the exact things that get scrapers IP-banned or make
headlines: missing rate limiting, no `User-Agent` identification, and requests
that ignore the target site's `robots.txt`. It runs entirely in your browser —
a Rust engine compiled to WebAssembly does the analysis, so nothing you paste
ever leaves your machine.

## Why

Most "robots.txt checker" tools only look at the *target site*. They tell you
what a site allows, not whether *your code* is well-behaved. polite_bot scores
the scraper itself — the code you're about to run — against the practices that
keep scrapers polite: identify yourself, throttle your requests, and respect
the rules the site publishes.

## The wow moment

Paste a scraping loop. In under a second you get a red/yellow/green verdict,
the exact line missing a rate limit or header, and a one-click diff that adds
the recommended fix.

## Features

- **Static analysis of your code** — detects missing `User-Agent` headers,
  generic/default User-Agent strings (e.g. `python-requests/2.x`), and
  missing backoff/rate-limit calls (`time.sleep`, `asyncio.sleep`,
  `setTimeout`, `tenacity`/`@retry`-style retry decorators), with
  line-accurate findings rendered both as a list and as colored gutter
  markers in the editor.
- **A real robots.txt parser** — compiled from Rust to WebAssembly, not a
  regex hack. Understands `User-agent` grouping, `Allow`/`Disallow`
  precedence, and `Crawl-delay`. Paste a target path and robots.txt (or
  leave it empty for allow-all) to cross-check live.
- **One-click fixes** — a "fix" button on the missing-User-Agent finding
  reveals a diff and a "copy fixed code" button, for both Python `requests`
  and JS `fetch` call shapes.
- **Sample snippets** — "worst case" / "rate-limited only" / "polite"
  buttons load a scored example with zero typing.

## Planned features

- **Shareable reports** — copy a markdown summary of the verdict for a PR
  description or code review.

## Stack

- **Engine:** Rust, compiled to `wasm32-unknown-unknown` via `wasm-bindgen`.
  Pure static analysis + a from-scratch robots.txt parser — no network calls,
  no server.
- **Site:** a static, dependency-free HTML/CSS/JS front end that loads the
  wasm module directly. No build tooling required to run it, no backend to
  host.

## Local development

```sh
# run the Rust test suite
cargo test

# build the wasm module + bindings into site/pkg/
scripts/build-wasm.sh

# then open site/index.html in a browser
```

See [`docs/VISION.md`](docs/VISION.md) for the product vision and design
decisions, [`docs/BACKLOG.md`](docs/BACKLOG.md) for the build plan,
[`docs/DESIGN.md`](docs/DESIGN.md) for the visual direction, and
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for a map of the codebase.

## License

MIT — see [`LICENSE`](LICENSE).
