# wpm-rs demo — record with:  vhs assets/demo.tape
# Selects the 15-second timed mode via Settings (Down, Down, Enter to apply),
# types past the 15s mark to show the test auto-finishing, then exports a card.
# Deterministic text via WPM_SEED so the typed line is 100% correct.

Output assets/demo.gif
Output assets/demo.mp4

Set Shell "zsh"
Set FontSize 18
Set Width 1200
Set Height 700
Set Padding 24
Set Theme "Dracula"

# --- hidden setup: fix seed, alias, clear leftover config so settings cursor starts at the documented
#     default (25 words), clear screen ---
Hide
Type "export WPM_SEED=7"
Enter
Type "alias wpm-rs=./target/release/wpm-rs"
Enter
Type "rm -f ~/Library/Application\ Support/wpm-rs/config.json"
Enter
Type "clear"
Enter
Show

# --- launch (default mode from saved config) ---
Type "wpm-rs"
Sleep 300ms
Enter
Sleep 1.5s

# --- open settings, select "15 seconds" (default cursor starts at "25 words"), apply it ---
Tab
Sleep 800ms
Down
Sleep 300ms
Down
Sleep 300ms
Down
Sleep 700ms
Enter
Sleep 1s

# --- type continuously, stop short of the 15s mark, then let it time out naturally ---
Type@72ms "in these which too state fact who than but turn mean a on give between consider still because call after over well ask could lead to eye where group these of general life back new man move from"
Sleep 2.5s

# --- results screen, then export a result card ---
Type "e"
Sleep 2.5s

# --- quit ---
Escape
Sleep 500ms
