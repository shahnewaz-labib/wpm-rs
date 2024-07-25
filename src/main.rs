extern crate ncurses;

use std::fs::{self};

use ncurses::*;
use rand::prelude::*;
use serde_json::{Result, Value};
use std::time::SystemTime;

static FILENAME: &str = "quotes/english.json";
static WORD_COUNT: usize = 10;

fn read_json() -> Result<Vec<String>> {
    let file = fs::File::open(FILENAME).expect("File not found");
    let json: Value = serde_json::from_reader(file).expect("Error while reading JSON file");
    let words = json["words"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_str().unwrap().to_string())
        .collect::<Vec<String>>();

    Ok(words)
}

fn get_random_string(words: &Vec<String>, count: usize) -> String {
    let mut rng = thread_rng();
    let mut result = String::new();

    for _ in 0..count {
        let index = rng.gen_range(0..words.len());
        result.push_str(&words[index]);
        result.push(' ');
    }

    result
}

fn main() {
    let words: Vec<String> = read_json().unwrap();

    let mut quit = false;
    let mut pos = 0;

    let mut message: String = get_random_string(&words, WORD_COUNT);

    let mut start = SystemTime::now();
    let mut started = false;

    initscr();
    raw();

    keypad(stdscr(), true);
    noecho();

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    scrollok(stdscr(), true);

    start_color();
    init_pair(1, COLOR_GREEN, COLOR_BLACK);
    init_pair(2, COLOR_RED, COLOR_BLACK);

    while !quit {
        attron(COLOR_PAIR(1));
        addstr(&message[..pos]);
        attroff(COLOR_PAIR(1));

        attron(COLOR_PAIR(2));
        addstr(&message[pos..]);
        attroff(COLOR_PAIR(2));

        let elapsed = start.elapsed().unwrap().as_secs_f32();
        addstr("\n\n");
        addstr(&format!("Time elapsed : {} s\n", elapsed));
        addstr(&format!("Total words  : {}\n", WORD_COUNT));
        addstr(&format!("WPM          : {}\n", WORD_COUNT as f32 / elapsed * 60.0));

        // getch is blocking the screen update
        let mut ch = getch();
        if ch == 3 {
            quit = true;
        }

        if !started {
            start = SystemTime::now();
            started = true;
        }

        if pos == message.len() - 2 {
            // idk why but it's 2
            clear();

            let elapsed = start.elapsed().unwrap().as_secs_f32();
            addstr(&format!("Time elapsed : {} s\n", elapsed));
            addstr(&format!("Total words  : {}\n", WORD_COUNT));
            addstr(&format!("WPM          : {}\n", WORD_COUNT as f32 / elapsed * 60.0));
            started = false;
            addstr("\nPress SPACE to play again or CTRL-C to quit");
            pos = 0;

            ch = getch();
            if ch == 32 {
                clear();
                message = get_random_string(&words, WORD_COUNT);
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
