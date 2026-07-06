use crate::cli::Cli;
use crate::config::Config;
use crate::history::{History, HistoryEntry};
use crate::stats::{calculate_accuracy, calculate_wpm};
use crate::theme::Theme;
use crate::words::{
    load_code_snippets, load_quotes, load_words_for_language, CodeProvider, PracticeProvider,
    QuoteProvider, TextProvider, WordProvider,
};
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;
use unicode_segmentation::UnicodeSegmentation;

const QUOTE_FILE: &str = "quotes/english_quotes.json";
const CODE_SNIPPET_FILE: &str = "quotes/code_snippets.json";
const DEFAULT_MODE_INDEX: usize = 1; // 25 words by default
const TIMED_WORD_POOL: usize = 300; // enough words so timed modes don't run out

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestMode {
    WordCount(usize),
    Timed(u64), // seconds
}

impl TestMode {
    pub fn label(&self) -> String {
        match self {
            TestMode::WordCount(n) => format!("{} words", n),
            TestMode::Timed(s) => format!("{} seconds", s),
        }
    }

    fn words_to_generate(&self) -> usize {
        match self {
            TestMode::WordCount(n) => *n,
            TestMode::Timed(_) => TIMED_WORD_POOL,
        }
    }

    pub fn is_timed(&self) -> bool {
        matches!(self, TestMode::Timed(_))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextSource {
    Words,
    Quotes,
    Code,
    Practice,
}

impl TextSource {
    pub fn label(&self) -> &'static str {
        match self {
            TextSource::Words => "Words",
            TextSource::Quotes => "Quotes",
            TextSource::Code => "Code",
            TextSource::Practice => "Practice",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    English,
    Spanish,
    Code,
}

impl Language {
    pub fn label(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Spanish => "Spanish",
            Language::Code => "Code",
        }
    }
}

pub const TEST_MODES: [TestMode; 8] = [
    TestMode::WordCount(10),
    TestMode::WordCount(25),
    TestMode::WordCount(50),
    TestMode::WordCount(100),
    TestMode::Timed(15),
    TestMode::Timed(30),
    TestMode::Timed(60),
    TestMode::Timed(120),
];

const PUNCTUATION_CURSOR_INDEX: usize = TEST_MODES.len();
const TEXT_SOURCE_CURSOR_INDEX: usize = TEST_MODES.len() + 1;
const LANGUAGE_CURSOR_INDEX: usize = TEST_MODES.len() + 2;
const BLIND_MODE_CURSOR_INDEX: usize = TEST_MODES.len() + 3;
const STOP_ON_ERROR_CURSOR_INDEX: usize = TEST_MODES.len() + 4;
const THEME_CURSOR_INDEX: usize = TEST_MODES.len() + 5;
const SOUND_CURSOR_INDEX: usize = TEST_MODES.len() + 6;
const SETTINGS_ITEM_COUNT: usize = TEST_MODES.len() + 7;

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
    History,
}

#[derive(Debug, Clone)]
pub struct TypedChar {
    pub expected: String,
    pub actual: Option<char>,
}

impl TypedChar {
    pub fn is_correct(&self) -> bool {
        self.actual
            .map(|a| self.expected.chars().next().map(|c| c == a).unwrap_or(false))
            .unwrap_or(false)
    }

    pub fn is_typed(&self) -> bool {
        self.actual.is_some()
    }
}

#[derive(Debug, Clone, Default)]
pub struct KeyStat {
    pub total_time_ms: u128,
    pub press_count: usize,
    pub error_count: usize,
}

impl KeyStat {
    pub fn avg_time_ms(&self) -> f64 {
        if self.press_count == 0 {
            0.0
        } else {
            self.total_time_ms as f64 / self.press_count as f64
        }
    }

    #[allow(dead_code)]
    pub fn error_rate(&self) -> f64 {
        if self.press_count == 0 {
            0.0
        } else {
            self.error_count as f64 / self.press_count as f64 * 100.0
        }
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
    quotes: Vec<String>,
    code_snippets: Vec<String>,
    pub test_mode: TestMode,
    pub should_quit: bool,
    pub total_keystrokes: usize,
    pub correct_keystrokes: usize,
    pub view_state: ViewState,
    pub settings_cursor: usize,
    pub history: History,
    pub punctuation: bool,
    pub text_source: TextSource,
    pub language: Language,
    pub wpm_samples: Vec<u64>,
    last_sample_time: Option<Instant>,
    pub blind_mode: bool,
    pub stop_on_error: bool,
    pub theme: Theme,
    pub sound_enabled: bool,
    pub key_stats: HashMap<char, KeyStat>,
    last_key_time: Option<Instant>,
    pub saved_key_stats: HashMap<char, KeyStat>,
    pub export_message: Option<String>,
}

impl App {
    pub fn new(cli: &Cli) -> Result<Self> {
        let config = Config::load()?;
        let test_mode = cli
            .test_mode()
            .or(config.test_mode)
            .unwrap_or(TEST_MODES[DEFAULT_MODE_INDEX]);
        let punctuation = if cli.punctuation {
            true
        } else {
            config.punctuation.unwrap_or(false)
        };
        let text_source = cli
            .text_source()
            .or(config.text_source)
            .unwrap_or(TextSource::Words);
        let language = cli
            .language()
            .or(config.language)
            .unwrap_or(Language::English);
        let blind_mode = if cli.blind {
            true
        } else {
            config.blind_mode.unwrap_or(false)
        };
        let stop_on_error = if cli.stop_on_error {
            true
        } else {
            config.stop_on_error.unwrap_or(false)
        };
        let theme = config.theme.unwrap_or_default();
        let sound_enabled = config.sound_enabled.unwrap_or(false);
        let words = load_words_for_language(language)?;
        let quotes = load_quotes(QUOTE_FILE)?;
        let code_snippets = load_code_snippets(CODE_SNIPPET_FILE)?;
        let target_text = Self::generate_target_text(&words, &quotes, &code_snippets, test_mode, punctuation, text_source, &HashMap::new());
        let typed_chars = target_text
            .graphemes(true)
            .map(|g| TypedChar {
                expected: g.to_string(),
                actual: None,
            })
            .collect();

        let settings_cursor = TEST_MODES
            .iter()
            .position(|m| *m == test_mode)
            .unwrap_or(DEFAULT_MODE_INDEX);
        let history = History::load()?;

        Ok(Self {
            target_text,
            typed_chars,
            cursor_pos: 0,
            state: GameState::NotStarted,
            start_time: None,
            end_time: None,
            words,
            quotes,
            code_snippets,
            test_mode,
            should_quit: false,
            total_keystrokes: 0,
            correct_keystrokes: 0,
            view_state: ViewState::Typing,
            settings_cursor,
            history,
            punctuation,
            text_source,
            language,
            wpm_samples: Vec::new(),
            last_sample_time: None,
            blind_mode,
            stop_on_error,
            theme,
            sound_enabled,
            key_stats: HashMap::new(),
            last_key_time: None,
            saved_key_stats: HashMap::new(),
            export_message: None,
        })
    }

    fn generate_target_text(
        words: &[String],
        quotes: &[String],
        code_snippets: &[String],
        mode: TestMode,
        punctuation: bool,
        text_source: TextSource,
        saved_key_stats: &HashMap<char, KeyStat>,
    ) -> String {
        let count = mode.words_to_generate();
        match text_source {
            TextSource::Words => WordProvider::new(words, punctuation).generate(count),
            TextSource::Quotes => QuoteProvider::new(quotes).generate(count),
            TextSource::Code => CodeProvider::new(code_snippets).generate(count),
            TextSource::Practice => {
                PracticeProvider::new(words, saved_key_stats).generate(count)
            }
        }
    }

    pub fn reset(&mut self) {
        self.target_text = Self::generate_target_text(
            &self.words,
            &self.quotes,
            &self.code_snippets,
            self.test_mode,
            self.punctuation,
            self.text_source,
            &self.saved_key_stats,
        );
        self.typed_chars = self
            .target_text
            .graphemes(true)
            .map(|g| TypedChar {
                expected: g.to_string(),
                actual: None,
            })
            .collect();
        self.cursor_pos = 0;
        self.state = GameState::NotStarted;
        self.start_time = None;
        self.end_time = None;
        self.total_keystrokes = 0;
        self.correct_keystrokes = 0;
        self.wpm_samples.clear();
        self.last_sample_time = None;
        self.key_stats.clear();
        self.last_key_time = None;
        self.export_message = None;
    }

    pub fn toggle_settings(&mut self) {
        match self.view_state {
            ViewState::Typing => {
                if self.state != GameState::Running {
                    self.view_state = ViewState::Settings;
                    self.settings_cursor = TEST_MODES
                        .iter()
                        .position(|m| *m == self.test_mode)
                        .unwrap_or(0);
                }
            }
            ViewState::Settings => {
                self.view_state = ViewState::Typing;
            }
            ViewState::History => {
                self.view_state = ViewState::Typing;
            }
        }
    }

    pub fn toggle_history(&mut self) {
        match self.view_state {
            ViewState::Typing => {
                if self.state != GameState::Running {
                    self.view_state = ViewState::History;
                }
            }
            ViewState::History => {
                self.view_state = ViewState::Typing;
            }
            ViewState::Settings => {
                self.view_state = ViewState::History;
            }
        }
    }

    pub fn settings_up(&mut self) {
        self.settings_cursor = self.settings_cursor.saturating_sub(1);
    }

    pub fn settings_down(&mut self) {
        self.settings_cursor = (self.settings_cursor + 1).min(SETTINGS_ITEM_COUNT - 1);
    }

    pub fn apply_settings(&mut self) {
        if self.settings_cursor == PUNCTUATION_CURSOR_INDEX {
            self.toggle_punctuation();
            return;
        }
        if self.settings_cursor == TEXT_SOURCE_CURSOR_INDEX {
            self.toggle_text_source();
            return;
        }
        if self.settings_cursor == LANGUAGE_CURSOR_INDEX {
            self.toggle_language();
            return;
        }
        if self.settings_cursor == BLIND_MODE_CURSOR_INDEX {
            self.blind_mode = !self.blind_mode;
            self.save_config();
            self.reset();
            return;
        }
        if self.settings_cursor == STOP_ON_ERROR_CURSOR_INDEX {
            self.stop_on_error = !self.stop_on_error;
            self.save_config();
            self.reset();
            return;
        }
        if self.settings_cursor == THEME_CURSOR_INDEX {
            self.theme = match self.theme {
                Theme::Dark => Theme::Light,
                Theme::Light => Theme::Retro,
                Theme::Retro => Theme::Dark,
            };
            self.save_config();
            return;
        }
        if self.settings_cursor == SOUND_CURSOR_INDEX {
            self.sound_enabled = !self.sound_enabled;
            self.save_config();
            return;
        }
        self.test_mode = TEST_MODES[self.settings_cursor];
        self.view_state = ViewState::Typing;
        self.reset();
        self.save_config();
    }

    pub fn toggle_punctuation(&mut self) {
        self.punctuation = !self.punctuation;
        self.save_config();
        self.reset();
    }

    pub fn toggle_text_source(&mut self) {
        self.text_source = match self.text_source {
            TextSource::Words => TextSource::Quotes,
            TextSource::Quotes => TextSource::Code,
            TextSource::Code => TextSource::Practice,
            TextSource::Practice => TextSource::Words,
        };
        self.save_config();
        self.reset();
    }

    pub fn toggle_language(&mut self) {
        self.language = match self.language {
            Language::English => Language::Spanish,
            Language::Spanish => Language::Code,
            Language::Code => Language::English,
        };
        self.words = load_words_for_language(self.language).unwrap_or_default();
        self.save_config();
        self.reset();
    }

    pub fn is_on_punctuation_toggle(&self) -> bool {
        self.settings_cursor == PUNCTUATION_CURSOR_INDEX
    }

    pub fn is_on_text_source_toggle(&self) -> bool {
        self.settings_cursor == TEXT_SOURCE_CURSOR_INDEX
    }

    pub fn is_on_language_toggle(&self) -> bool {
        self.settings_cursor == LANGUAGE_CURSOR_INDEX
    }

    pub fn is_on_blind_mode_toggle(&self) -> bool {
        self.settings_cursor == BLIND_MODE_CURSOR_INDEX
    }

    pub fn is_on_stop_on_error_toggle(&self) -> bool {
        self.settings_cursor == STOP_ON_ERROR_CURSOR_INDEX
    }

    pub fn is_on_theme_toggle(&self) -> bool {
        self.settings_cursor == THEME_CURSOR_INDEX
    }

    pub fn is_on_sound_toggle(&self) -> bool {
        self.settings_cursor == SOUND_CURSOR_INDEX
    }

    fn save_config(&self) {
        let config = Config {
            test_mode: Some(self.test_mode),
            punctuation: Some(self.punctuation),
            text_source: Some(self.text_source),
            language: Some(self.language),
            blind_mode: Some(self.blind_mode),
            stop_on_error: Some(self.stop_on_error),
            theme: Some(self.theme),
            sound_enabled: Some(self.sound_enabled),
        };
        let _ = config.save();
    }

    fn save_result(&mut self) {
        self.saved_key_stats = self.key_stats.clone();
        let entry = HistoryEntry {
            wpm: self.net_wpm(),
            raw_wpm: self.wpm(),
            accuracy: self.accuracy(),
            elapsed_secs: self.elapsed_secs(),
            test_mode: self.test_mode,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0),
        };
        self.history.add_entry(entry);
    }

    pub fn check_time_expired(&mut self) {
        if let TestMode::Timed(secs) = self.test_mode {
            if self.state == GameState::Running && self.elapsed_secs() >= secs as f64 {
                self.state = GameState::Finished;
                self.end_time = Some(Instant::now());
                self.save_result();
            }
        }
    }

    pub fn maybe_sample_wpm(&mut self) {
        if self.state != GameState::Running {
            return;
        }
        let should_sample = match self.last_sample_time {
            Some(last) => last.elapsed().as_secs() >= 1,
            None => true,
        };
        if should_sample {
            self.wpm_samples.push(self.net_wpm().round() as u64);
            self.last_sample_time = Some(Instant::now());
        }
    }

    pub fn time_remaining(&self) -> f64 {
        match self.test_mode {
            TestMode::Timed(secs) => (secs as f64 - self.elapsed_secs()).max(0.0),
            _ => 0.0,
        }
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
            let expected_first = typed.expected.chars().next().unwrap_or('\0');

            if self.stop_on_error && expected_first != c {
                self.total_keystrokes += 1;
                self.play_error_sound();
                self.record_key_stat(expected_first, c, false);
                return;
            }

            typed.actual = Some(c);

            self.total_keystrokes += 1;
            let is_correct = typed.is_correct();
            if is_correct {
                self.correct_keystrokes += 1;
            } else {
                self.play_error_sound();
            }

            self.record_key_stat(expected_first, c, is_correct);

            self.cursor_pos += 1;

            if self.cursor_pos == self.typed_chars.len() {
                self.state = GameState::Finished;
                self.end_time = Some(Instant::now());
                self.save_result();
            }
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor_pos > 0 && self.state != GameState::Finished {
            self.cursor_pos -= 1;
            let typed = &mut self.typed_chars[self.cursor_pos];

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

    pub fn net_wpm(&self) -> f64 {
        let duration = match (self.start_time, self.end_time) {
            (Some(start), Some(end)) => end.duration_since(start),
            (Some(start), None) => start.elapsed(),
            _ => return 0.0,
        };

        calculate_wpm(self.correct_keystrokes, duration)
    }

    pub fn accuracy(&self) -> f64 {
        calculate_accuracy(self.correct_keystrokes, self.total_keystrokes)
    }

    pub fn source_label(&self) -> String {
        match self.text_source {
            TextSource::Words => self.language.label().to_string(),
            TextSource::Quotes => "Quotes".to_string(),
            TextSource::Code => "Code".to_string(),
            TextSource::Practice => "Practice".to_string(),
        }
    }

    fn play_error_sound(&self) {
        if self.sound_enabled {
            use std::io::Write;
            let mut stdout = std::io::stdout();
            let _ = stdout.write_all(b"\x07");
            let _ = stdout.flush();
        }
    }

    fn record_key_stat(&mut self, expected: char, _actual: char, is_correct: bool) {
        let now = Instant::now();
        let interval_ms = self
            .last_key_time
            .map(|last| now.duration_since(last).as_millis())
            .unwrap_or(0);

        let stat = self.key_stats.entry(expected).or_default();
        stat.total_time_ms += interval_ms;
        stat.press_count += 1;
        if !is_correct {
            stat.error_count += 1;
        }

        self.last_key_time = Some(now);
    }

    pub fn slowest_keys(&self, n: usize) -> Vec<(char, KeyStat)> {
        let mut keys: Vec<(char, KeyStat)> = self
            .key_stats
            .iter()
            .filter(|(_, s)| s.press_count >= 2)
            .map(|(k, s)| (*k, s.clone()))
            .collect();
        keys.sort_by(|a, b| {
            b.1.avg_time_ms()
                .partial_cmp(&a.1.avg_time_ms())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        keys.truncate(n);
        keys
    }

    pub fn most_error_prone_keys(&self, n: usize) -> Vec<(char, KeyStat)> {
        let mut keys: Vec<(char, KeyStat)> = self
            .key_stats
            .iter()
            .filter(|(_, s)| s.error_count > 0)
            .map(|(k, s)| (*k, s.clone()))
            .collect();
        keys.sort_by_key(|(_, s)| std::cmp::Reverse(s.error_count));
        keys.truncate(n);
        keys
    }

    pub fn export_result(&mut self) {
        let data_dir = match dirs::data_dir() {
            Some(d) => d.join("wpm-rs"),
            None => {
                self.export_message = Some("Error: Could not find data directory".to_string());
                return;
            }
        };
        if let Err(e) = std::fs::create_dir_all(&data_dir) {
            self.export_message = Some(format!("Error: {}", e));
            return;
        }

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let filename = format!("result_{}.txt", timestamp);
        let filepath = data_dir.join(&filename);

        let net = self.net_wpm();
        let raw = self.wpm();
        let acc = self.accuracy();
        let time = self.elapsed_secs();
        let mode = self.test_mode.label();
        let source = self.source_label();
        let best = self.history.best_wpm(self.test_mode);

        let slowest = self.slowest_keys(5);
        let slowest_str = if slowest.is_empty() {
            "N/A".to_string()
        } else {
            slowest
                .iter()
                .map(|(k, s)| {
                    let display = if *k == ' ' { "spc".to_string() } else { k.to_string() };
                    format!("{} ({:.0}ms)", display, s.avg_time_ms())
                })
                .collect::<Vec<_>>()
                .join(", ")
        };

        let errors = self.most_error_prone_keys(5);
        let errors_str = if errors.is_empty() {
            "None".to_string()
        } else {
            errors
                .iter()
                .map(|(k, s)| {
                    let display = if *k == ' ' { "spc".to_string() } else { k.to_string() };
                    format!("{} ({} errors)", display, s.error_count)
                })
                .collect::<Vec<_>>()
                .join(", ")
        };

        let card = format!(
            "================================\n\
             WPM-RS RESULT CARD\n\
             =================================\n\
             Mode:          {}\n\
             Source:        {}\n\
             Net WPM:       {:.0}\n\
             Raw WPM:       {:.0}\n\
             Accuracy:      {:.1}%\n\
             Time:          {:.1}s\n\
             Personal Best: {}\n\
             --------------------------------\n\
             Slowest Keys:  {}\n\
             Error-Prone:   {}\n\
             =================================\n",
            mode,
            source,
            net,
            raw,
            acc,
            time,
            best.map(|b| format!("{:.0} WPM", b)).unwrap_or("N/A".to_string()),
            slowest_str,
            errors_str,
        );

        match std::fs::write(&filepath, &card) {
            Ok(_) => {
                self.export_message = Some(format!(
                    "Exported to {}",
                    filepath.display()
                ));
            }
            Err(e) => {
                self.export_message = Some(format!("Error: {}", e));
            }
        }
    }
}
