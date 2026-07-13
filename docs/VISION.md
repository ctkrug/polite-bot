# Vision

## The problem

People write scraping scripts fast, and the code that gets written fast is
rarely polite: no `User-Agent`, no delay between requests, no check of the
target's `robots.txt`. The first sign anything's wrong is usually a 429, an
IP ban, or — for anyone scraping at any real scale — an abuse complaint
against their host. By then the damage is done and the fix is a guess.

Existing "robots.txt checker" tools only look at the target site. They'll
tell you a path is disallowed, but they say nothing about *your code* — the
thing you're actually about to run. There's no tool that reads the scraper
itself and tells you, before you run it, exactly what's missing.

## Who it's for

Developers writing one-off or early-stage scraping/crawling scripts —
personal projects, data-collection scripts for research, internal tooling —
who want a fast sanity check before they hit "run." Not an enterprise
compliance tool; a paste-and-check gut check, the kind of thing you'd reach
for the way you'd run a linter before a commit.

## The core idea

Paste a scraper snippet. A static analyzer (compiled from Rust to
WebAssembly, running entirely client-side) scans the source for the concrete
signals of a polite scraper — a declared identity, some form of throttling —
and combines that with a real robots.txt parser so the tool can also check
whether the *target* permits what the code is about to do. The result is a
single red/yellow/green verdict with line-accurate findings, not a wall of
disclaimers.

## Key design decisions

- **Client-side only.** The analysis engine is Rust compiled to wasm and runs
  in the browser. Nothing pasted is ever sent to a server — this matters
  both for trust (people paste API keys and internal URLs into scrapers) and
  for cost (a static site with zero backend is free to host and to run).
- **Score the code, not just the target.** This is the tool's actual edge
  over every "robots.txt checker": most existing tools only answer "does the
  site allow this," never "is my code well-behaved." Both checks matter, but
  scoring the scraper's own source is the differentiator worth building
  first — see `docs/BACKLOG.md` Epic 1.
- **Real parsing, not regex-guessing.** The robots.txt parser implements
  actual `User-agent` grouping and `Allow`/`Disallow` precedence (longest
  match wins), because a tool that tells developers to trust its verdict
  needs to get the spec right.
- **Actionable over informational.** A finding without a fix is a lecture. A
  finding with a one-click diff is a tool. Every finding in v1 should point
  at both what's wrong and what to paste instead.

## What "v1 done" looks like

- Paste a Python `requests`-style scraper loop and get a verdict (red/
  yellow/green) within ~300ms, entirely client-side.
- Every finding names the exact line and explains the risk in one sentence.
- At least one finding type (missing `User-Agent`) offers a one-click diff
  that inserts the fix.
- A real robots.txt parser is wired in so a pasted or fetched robots.txt can
  flag scraped paths that are disallowed.
- The site is a single static bundle (`site/`), deployable to
  `apps.charliekrug.com/polite-bot` with relative asset paths and no backend.
- The page matches the direction in `docs/DESIGN.md` — this is a linter,
  not a form, and the UI should feel like one.

See `docs/BACKLOG.md` for the epic/story breakdown that gets there.
