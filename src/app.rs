use crate::stats::{calculate_accuracy, calculate_wpm};
use crate::words::{generate_text, load_words};
use color_eyre::Result;
use std::time::Instant;

const WORD_FILE: &str = "quotes/english.json";
const DEFAULT_WORD_COUNT: usize = 10;

pub const WORD_COUNT_OPTIONS: [usize; 4] = [10, 25, 50, 100];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    NotStarted,
    Running,
    Finished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewState {
    Typing,
    Settings,
}

#[derive(Debug, Clone)]
pub struct TypedChar {
    pub expected: char,
    pub actual: Option<char>,
}

impl TypedChar {
    pub fn is_correct(&self) -> bool {
        self.actual.map(|a| a == self.expected).unwrap_or(false)
    }

    pub fn is_typed(&self) -> bool {
        self.actual.is_some()
    }
}

pub struct App {
    pub target_text: String,
    pub typed_chars: Vec<TypedChar>,
    pub cursor_pos: usize,
    pub state: GameState,
    pub start_time: Option<Instant>,
    pub end_time: Option<Instant>,
    words: Vec<String>,
    pub word_count: usize,
    pub should_quit: bool,
    pub total_keystrokes: usize,
    pub correct_keystrokes: usize,
    pub view_state: ViewState,
    pub settings_cursor: usize,
}

impl App {
    pub fn new() -> Result<Self> {
        let words = load_words(WORD_FILE)?;
        let word_count = DEFAULT_WORD_COUNT;
        let target_text = generate_text(&words, word_count);
        let typed_chars = target_text
            .chars()
            .map(|c| TypedChar {
                expected: c,
                actual: None,
            })
            .collect();

        // Find initial cursor position for settings (matching current word_count)
        let settings_cursor = WORD_COUNT_OPTIONS
            .iter()
            .position(|&w| w == word_count)
            .unwrap_or(0);

        Ok(Self {
            target_text,
            typed_chars,
            cursor_pos: 0,
            state: GameState::NotStarted,
            start_time: None,
            end_time: None,
            words,
            word_count,
            should_quit: false,
            total_keystrokes: 0,
            correct_keystrokes: 0,
            view_state: ViewState::Typing,
            settings_cursor,
        })
    }

    pub fn reset(&mut self) {
        self.target_text = generate_text(&self.words, self.word_count);
        self.typed_chars = self
            .target_text
            .chars()
            .map(|c| TypedChar {
                expected: c,
                actual: None,
            })
            .collect();
        self.cursor_pos = 0;
        self.state = GameState::NotStarted;
        self.start_time = None;
        self.end_time = None;
        self.total_keystrokes = 0;
        self.correct_keystrokes = 0;
    }

    pub fn toggle_settings(&mut self) {
        match self.view_state {
            ViewState::Typing => {
                if self.state != GameState::Running {
                    self.view_state = ViewState::Settings;
                    // Sync cursor to current word_count
                    self.settings_cursor = WORD_COUNT_OPTIONS
                        .iter()
                        .position(|&w| w == self.word_count)
                        .unwrap_or(0);
                }
            }
            ViewState::Settings => {
                self.view_state = ViewState::Typing;
            }
        }
    }

    pub fn settings_up(&mut self) {
        self.settings_cursor = self.settings_cursor.saturating_sub(1);
    }

    pub fn settings_down(&mut self) {
        self.settings_cursor = (self.settings_cursor + 1).min(WORD_COUNT_OPTIONS.len() - 1);
    }

    pub fn apply_settings(&mut self) {
        self.word_count = WORD_COUNT_OPTIONS[self.settings_cursor];
        self.view_state = ViewState::Typing;
        self.reset();
    }

    pub fn type_char(&mut self, c: char) {
        if self.state == GameState::Finished {
            return;
        }

        if self.state == GameState::NotStarted {
            self.state = GameState::Running;
            self.start_time = Some(Instant::now());
        }

        if self.cursor_pos < self.typed_chars.len() {
            let typed = &mut self.typed_chars[self.cursor_pos];
            typed.actual = Some(c);

            self.total_keystrokes += 1;
            if typed.is_correct() {
                self.correct_keystrokes += 1;
            }

            self.cursor_pos += 1;

            if self.cursor_pos == self.typed_chars.len() {
                self.state = GameState::Finished;
                self.end_time = Some(Instant::now());
            }
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor_pos > 0 && self.state != GameState::Finished {
            self.cursor_pos -= 1;
            let typed = &mut self.typed_chars[self.cursor_pos];

            // Adjust keystroke counts when deleting
            if typed.is_typed() {
                self.total_keystrokes = self.total_keystrokes.saturating_sub(1);
                if typed.is_correct() {
                    self.correct_keystrokes = self.correct_keystrokes.saturating_sub(1);
                }
            }

            typed.actual = None;
        }
    }

    pub fn elapsed_secs(&self) -> f64 {
        match (self.start_time, self.end_time) {
            (Some(start), Some(end)) => end.duration_since(start).as_secs_f64(),
            (Some(start), None) => start.elapsed().as_secs_f64(),
            _ => 0.0,
        }
    }

    pub fn wpm(&self) -> f64 {
        let duration = match (self.start_time, self.end_time) {
            (Some(start), Some(end)) => end.duration_since(start),
            (Some(start), None) => start.elapsed(),
            _ => return 0.0,
        };

        calculate_wpm(self.cursor_pos, duration)
    }

    pub fn accuracy(&self) -> f64 {
        calculate_accuracy(self.correct_keystrokes, self.total_keystrokes)
    }
}
