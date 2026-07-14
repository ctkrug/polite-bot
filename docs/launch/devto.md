---
title: "I built a scraper politeness checker that runs entirely in your browser"
published: false
tags: rust, webassembly, webscraping, showdev
---

I scrape the web a fair amount for side projects, and every time I write a fresh
`requests` loop I make the same mistakes: no `User-Agent`, no delay between
requests, no glance at the target's `robots.txt`. The first sign anything is
wrong is usually a 429 or an IP ban, and by then the fix is a guess.

Every "robots.txt checker" I could find only looks at the target site. None of
them read the thing I am actually about to run: my own code. So I built
[polite_bot](https://apps.charliekrug.com/polite-bot/), a tool you paste a
scraper into that scores it red, yellow, or green and points at the exact line
to fix. It runs completely client-side.

## Why Rust and WebAssembly

The privacy angle decided the architecture for me. People paste scrapers that
contain internal URLs and sometimes API keys, and I did not want any of that
touching a server. If there is no backend, there is nothing to leak and nothing
to host. So the whole analyzer is a Rust crate compiled to
`wasm32-unknown-unknown` with `wasm-bindgen`, and the site is static HTML, CSS,
and one JS module that loads the wasm. No build step to run it, no server to pay
for.

Rust also made the one part I cared about getting right easy to trust: the
robots.txt parser.

## The robots.txt parser was the interesting part

A robots.txt file looks trivial until you implement it. The rule that trips
people up is precedence: when both an `Allow` and a `Disallow` match a path, the
*longer* (more specific) rule wins, and a tie goes to `Allow`. Groups also
matter: a `User-agent: *` block is only the fallback when no named group matches
the crawler you are asking about.

I wrote it from scratch rather than pulling a crate, because a tool that tells
you to trust its verdict needs to get the spec right, and I wanted every case
covered by a test I could point at:

```rust
match (best_disallow, best_allow) {
    (Some(d), Some(a)) => a >= d, // longest match wins; tie favors Allow
    (Some(_), None) => false,
    _ => true,
}
```

Empty or malformed input parses to zero groups, which resolves to allow-all
instead of an error state. That mattered for the paste box: people paste
half a file, and a crash there would be a worse experience than a best-effort
answer.

## The one-click fix was harder than the analysis

Detecting a missing `User-Agent` is a substring scan. Generating a patch that
inserts one without corrupting the code is where the bugs lived. The fixer finds
the call's opening paren, walks the string bracket-depth-aware to the matching
close paren so nested calls like `requests.get(build_url(x))` still patch
correctly, and declines rather than guesses when the call already passes
options.

The nastiest bug I found in QA: the analyzer would happily "fix" a `fetch(` that
was actually text inside a print statement or a docstring, mangling the string.
The fix was a small scan that checks whether the matched call sits inside an
unescaped string literal earlier on the line, and bails if so. There is a
regression test for it now, alongside a fuzz test that throws random bytes and
multibyte UTF-8 at the fixer to prove it never panics on a bad slice.

## What I would do differently

The analysis is heuristic: it scans for known request-call and rate-limit
signals rather than parsing an AST. That is fast and dependency-free, but it
means unusual code can slip through. Instead of guessing red on source it does
not recognize, it degrades to a yellow "check this manually" verdict, which felt
like the honest default. If I take this further, a real Python and JavaScript
parser would let me catch a lot more without false positives.

## Try it

Paste a scraper and see what it says:
[apps.charliekrug.com/polite-bot](https://apps.charliekrug.com/polite-bot/).
The code is on [GitHub](https://github.com/ctkrug/polite-bot). Feedback on the
detection rules is very welcome, especially cases where it gets a verdict wrong.
