use crate::app::{Language, TestMode, TextSource};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "wpm-rs", about = "A terminal-based typing speed test", version)]
pub struct Cli {
    #[arg(short, long, help = "Number of words for word-count mode")]
    pub words: Option<usize>,

    #[arg(short, long, help = "Time in seconds for timed mode")]
    pub time: Option<u64>,

    #[arg(short, long, help = "Language: english, spanish, code")]
    pub lang: Option<String>,

    #[arg(long, help = "Enable punctuation mode")]
    pub punctuation: bool,

    #[arg(long, help = "Use quotes as text source")]
    pub quotes: bool,

    #[arg(long, help = "Use code snippets as text source")]
    pub code: bool,

    #[arg(long, help = "Enable blind mode")]
    pub blind: bool,

    #[arg(long, help = "Enable stop-on-error mode")]
    pub stop_on_error: bool,

    // ponytail: print generated target text and exit (demos/tests). Hidden.
    #[arg(long, hide = true)]
    pub print_text: bool,
}

impl Cli {
    pub fn test_mode(&self) -> Option<TestMode> {
        if let Some(w) = self.words {
            return Some(TestMode::WordCount(w));
        }
        self.time.map(TestMode::Timed)
    }

    pub fn language(&self) -> Option<Language> {
        self.lang.as_deref().map(|s| match s.to_lowercase().as_str() {
            "english" | "en" => Language::English,
            "spanish" | "es" => Language::Spanish,
            "code" => Language::Code,
            _ => Language::English,
        })
    }

    pub fn text_source(&self) -> Option<TextSource> {
        if self.quotes {
            Some(TextSource::Quotes)
        } else if self.code {
            Some(TextSource::Code)
        } else {
            None
        }
    }
}
