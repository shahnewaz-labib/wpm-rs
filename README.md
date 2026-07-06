# wpm-rs

A terminal-based typing speed test written in Rust with `ratatui`.

## Quick Start

```bash
cargo run
cargo run --release   # faster build
```

## Usage

```bash
cargo run -- --words 50          # 50-word test
cargo run -- --time 60           # 60-second timed test
cargo run -- --lang spanish      # Spanish word list
cargo run -- --code              # Code snippet mode
cargo run -- --quotes            # Quote mode
cargo run -- --punctuation       # With capitals & punctuation
cargo run -- --blind             # Blind mode (no feedback)
cargo run -- --stop-on-error     # Can't advance on wrong keys
wpm-rs --help                    # Show all options
```

## Key Bindings

| Key | Action |
|---|---|
| Type to begin | Start test |
| Tab | Open settings |
| F2 | Open history |
| E | Export result card (when finished) |
| Space | Play again (when finished) |
| Backspace | Delete last character |
| Esc / Ctrl+C | Quit |

## Settings

| Setting | Values |
|---|---|
| Test Mode | 10/25/50/100 words, 15/30/60/120 seconds |
| Punctuation | Off / On |
| Text Source | Words / Quotes / Code / Practice |
| Language | English / Spanish / Code |
| Blind Mode | Off / On |
| Stop on Error | Off / On |
| Theme | Dark / Light / Retro |
| Sound | Off / On |

Settings persist across sessions in `~/.config/wpm-rs/config.json`.

## History

Past results are saved in `~/.local/share/wpm-rs/history.json` (XDG data dir).
Personal best WPM per mode is shown on the results screen.
Press `E` on the results screen to export a result card.
Press `F2` to view history with WPM trend chart.

## Features

- Timed and word-count test modes
- Net WPM vs raw WPM (industry-standard scoring)
- Live WPM sparkline during typing
- Word-level error highlighting (monkeytype-style)
- Punctuation & capitalization mode
- Quote mode with 50 famous quotes
- Code-typing mode with 20 Rust snippets
- Multi-language word lists
- Practice/drill mode targeting weak keys
- Per-character speed/error heat-map
- Blind mode and stop-on-error toggles
- Dark/Light/Retro themes
- Optional sound feedback
- Unicode grapheme-safe typing
