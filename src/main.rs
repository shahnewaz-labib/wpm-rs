extern crate ncurses;

use ncurses::*;
use rand::prelude::*;
use std::time::SystemTime;

// vector with 5 random long quotes
fn get_quote() -> Vec<&'static str> {
    vec![
        "a quick brown fox jumps over the lazy dog",
        "stay hungry, stay foolish."
    ]
}

fn main() {
    initscr();
    raw();

    keypad(stdscr(), true);
    noecho();

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    scrollok(stdscr(), true);

    start_color();
    init_pair(1, COLOR_GREEN, COLOR_BLACK);
    init_pair(2, COLOR_RED, COLOR_BLACK);

    let mut quit = false;
    let mut pos = 0;
    let mut rng = thread_rng();

    let quotes = get_quote();
    // get random quote and assign it to message
    let mut message = quotes[rng.gen_range(0..quotes.len())];

    let mut start = SystemTime::now();
    let mut started = false;

    while !quit {
        attron(COLOR_PAIR(1));
        addstr(&message[..pos]);
        attroff(COLOR_PAIR(1));

        attron(COLOR_PAIR(2));
        addstr(&message[pos..]);
        attroff(COLOR_PAIR(2));

        let mut ch = getch();
        if ch == 3 {
            quit = true;
        }

        if !started {
            start = SystemTime::now();
            started = true;
        }

        if pos == message.len() - 1 {
            clear();

            let words: Vec<&str> = message.split(' ').collect();
            let message_len: f32 = words.len() as f32;

            let elapsed = start.elapsed().unwrap().as_secs_f32();
            addstr(&format!("Time elapsed : {} s\n", elapsed));
            addstr(&format!("Total words  : {}\n", message_len));
            addstr(&format!("WPM          : {}\n", message_len / elapsed * 60.0));
            started = false;
            addstr("\nPress SPACE to play again or CTRL-C to quit");
            pos = 0;

            ch = getch();
            if ch == 32 {
                clear();
                message = quotes[rng.gen_range(0..quotes.len())];
                quit = false;
                continue;
            } else if ch == 3 {
                quit = true;
            }
        }

        if ch == message.as_bytes()[pos] as i32 {
            pos += 1;
        }

        clear();
        refresh();
    }

    reset_shell_mode();
    endwin();
}
