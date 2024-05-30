extern crate ncurses;

use ncurses::*;
use rand::prelude::*;
use std::time::SystemTime;

// vector with 5 random long quotes
fn get_quote() -> Vec<&'static str> {
    vec![
        "The only way to do great work is to love what you do. If you haven't found it yet, keep looking. Don't settle. As with all matters of the heart, you'll know when you find it.",
        "Your work is going to fill a large part of your life, and the only way to be truly satisfied is to do what you believe is great work. And the only way to do great work is to love what you do. If you haven't found it yet, keep looking. Don't settle. As with all matters of the heart, you'll know when you find it.",
        "Your time is limited, don't waste it living someone else's life. Don't be trapped by dogma, which is living the result of other people's thinking. Don't let the noise of other's opinion drowned your own inner voice. And most important, have the courage to follow your heart and intuition, they somehow already know what you truly want to become. Everything else is secondary.",
        "Don't be trapped by dogma - which is living with the results of other people's thinking. Don't let the noise of others' opinions drown out your own inner voice. And most important, have the courage to follow your heart and intuition.",
        "Stay hungry, stay foolish."
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
            addstr("You finished in ");
            let elapsed = start.elapsed().unwrap().as_secs_f32();
            addstr(&format!("{} s\n", elapsed));

            let message_len = message.len() as f32;

            // fix the calculation
            addstr(&format!("WPM: {}\n", message_len / elapsed * 60.0));
            started = false;

            addstr("Press SPACE to play again or CTRL-C to quit");
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
