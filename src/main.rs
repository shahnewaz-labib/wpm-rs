extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();
    raw();

    keypad(stdscr(), true);
    noecho();

    scrollok(stdscr(), true);

    start_color();
    init_pair(1, COLOR_GREEN, COLOR_BLACK);
    init_pair(2, COLOR_RED, COLOR_BLACK);

    let mut quit = false;
    let mut pos = 0;
    let mut message: &str = "a lazy brown fox jumps over the fence";

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

        if pos == message.len() - 1 {
            clear();
            addstr("Press SPACE to play again or CTRL-C to quit");
            pos = 0;

            ch = getch();
            if ch == 32 {
                clear();
                message = "a lazy brown fox jumps over the fence";
                quit = false;
                continue;
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
