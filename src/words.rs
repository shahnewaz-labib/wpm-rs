use crate::app::{KeyStat, Language};
use color_eyre::Result;
use rand::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

const DEFAULT_WORDS: &str = include_str!("../quotes/english.json");
const DEFAULT_SPANISH: &str = include_str!("../quotes/spanish.json");
const DEFAULT_CODE: &str = include_str!("../quotes/code.json");
const DEFAULT_QUOTES: &str = include_str!("../quotes/english_quotes.json");
const DEFAULT_SNIPPETS: &str = include_str!("../quotes/code_snippets.json");

#[derive(Debug, Deserialize)]
struct WordList {
    words: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct QuoteList {
    quotes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SnippetList {
    snippets: Vec<String>,
}

pub fn load_words_for_language(language: Language) -> Result<Vec<String>> {
    let (path, default) = match language {
        Language::English => ("quotes/english.json", DEFAULT_WORDS),
        Language::Spanish => ("quotes/spanish.json", DEFAULT_SPANISH),
        Language::Code => ("quotes/code.json", DEFAULT_CODE),
    };
    let content = fs::read_to_string(path).unwrap_or_else(|_| default.to_string());
    let list: WordList = serde_json::from_str(&content)?;
    Ok(list.words)
}

pub fn load_quotes(path: &str) -> Result<Vec<String>> {
    let content = fs::read_to_string(path).unwrap_or_else(|_| DEFAULT_QUOTES.to_string());
    let list: QuoteList = serde_json::from_str(&content)?;
    Ok(list.quotes)
}

pub fn load_code_snippets(path: &str) -> Result<Vec<String>> {
    let content = fs::read_to_string(path).unwrap_or_else(|_| DEFAULT_SNIPPETS.to_string());
    let list: SnippetList = serde_json::from_str(&content)?;
    Ok(list.snippets)
}

pub trait TextProvider {
    fn generate(&self, count: usize) -> String;
}

pub struct WordProvider<'a> {
    words: &'a [String],
    punctuation: bool,
}

impl<'a> WordProvider<'a> {
    pub fn new(words: &'a [String], punctuation: bool) -> Self {
        Self { words, punctuation }
    }
}

impl<'a> TextProvider for WordProvider<'a> {
    fn generate(&self, count: usize) -> String {
        generate_text(self.words, count, self.punctuation)
    }
}

pub struct QuoteProvider<'a> {
    quotes: &'a [String],
}

impl<'a> QuoteProvider<'a> {
    pub fn new(quotes: &'a [String]) -> Self {
        Self { quotes }
    }
}

impl<'a> TextProvider for QuoteProvider<'a> {
    fn generate(&self, count: usize) -> String {
        generate_quote_text(self.quotes, count)
    }
}

pub struct CodeProvider<'a> {
    snippets: &'a [String],
}

impl<'a> CodeProvider<'a> {
    pub fn new(snippets: &'a [String]) -> Self {
        Self { snippets }
    }
}

impl<'a> TextProvider for CodeProvider<'a> {
    fn generate(&self, count: usize) -> String {
        generate_code_text(self.snippets, count)
    }
}

pub struct PracticeProvider<'a> {
    words: &'a [String],
    key_stats: &'a HashMap<char, KeyStat>,
}

impl<'a> PracticeProvider<'a> {
    pub fn new(words: &'a [String], key_stats: &'a HashMap<char, KeyStat>) -> Self {
        Self { words, key_stats }
    }
}

impl<'a> TextProvider for PracticeProvider<'a> {
    fn generate(&self, count: usize) -> String {
        generate_practice_text(self.words, self.key_stats, count)
    }
}

pub fn generate_practice_text(
    words: &[String],
    key_stats: &HashMap<char, KeyStat>,
    count: usize,
) -> String {
    if words.is_empty() {
        return String::new();
    }

    let weak_keys: Vec<char> = {
        let mut stats: Vec<(char, &KeyStat)> = key_stats
            .iter()
            .map(|(&k, v)| (k, v))
            .filter(|(_, s)| s.press_count >= 2)
            .collect();
        stats.sort_by_key(|(_, s)| {
            std::cmp::Reverse((s.error_count as f64 * 10.0 + s.avg_time_ms()) as u64)
        });
        stats.iter().take(8).map(|(k, _)| *k).collect()
    };

    if weak_keys.is_empty() {
        return generate_text(words, count, false);
    }

    let filtered: Vec<&String> = words
        .iter()
        .filter(|w| weak_keys.iter().any(|k| w.contains(*k)))
        .collect();

    if filtered.is_empty() {
        return generate_text(words, count, false);
    }

    let mut rng = thread_rng();
    let mut result = String::new();
    for i in 0..count {
        let index = rng.gen_range(0..filtered.len());
        result.push_str(filtered[index]);
        if i < count - 1 {
            result.push(' ');
        }
    }
    result
}

pub fn generate_code_text(snippets: &[String], min_words: usize) -> String {
    if snippets.is_empty() {
        return String::new();
    }

    let mut rng = thread_rng();
    let mut result = String::new();
    let mut word_count = 0;

    while word_count < min_words {
        let snippet = &snippets[rng.gen_range(0..snippets.len())];
        if !result.is_empty() {
            result.push('\n');
        }
        result.push_str(snippet);
        word_count += snippet.split_whitespace().count();
    }

    result
}

pub fn generate_text(words: &[String], count: usize, punctuation: bool) -> String {
    let mut rng = thread_rng();
    let mut last_index: Option<usize> = None;

    let pick = |rng: &mut ThreadRng, last_index: &mut Option<usize>| -> usize {
        let mut index = rng.gen_range(0..words.len());
        while Some(index) == *last_index && words.len() > 1 {
            index = rng.gen_range(0..words.len());
        }
        *last_index = Some(index);
        index
    };

    if !punctuation {
        let mut result = String::new();
        for i in 0..count {
            let index = pick(&mut rng, &mut last_index);
            result.push_str(&words[index]);
            if i < count - 1 {
                result.push(' ');
            }
        }
        return result;
    }

    let mut result = String::new();
    let mut i = 0;
    while i < count {
        let remaining = count - i;
        let sentence_len = rng.gen_range(5..=12).min(remaining);
        let sentence_end = i + sentence_len;

        for j in i..sentence_end {
            let index = pick(&mut rng, &mut last_index);
            let word = &words[index];

            let formatted = if j == i {
                let mut chars = word.chars();
                match chars.next() {
                    Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
                    None => String::new(),
                }
            } else if word == "i" {
                "I".to_string()
            } else {
                word.clone()
            };

            result.push_str(&formatted);

            if j < sentence_end - 1 {
                if rng.gen_bool(0.15) {
                    result.push(',');
                }
                result.push(' ');
            }
        }

        let punct = match rng.gen_range(0..10) {
            0..=1 => '?',
            2..=3 => '!',
            _ => '.',
        };
        result.push(punct);

        if sentence_end < count {
            result.push(' ');
        }

        i = sentence_end;
    }

    result
}

pub fn generate_quote_text(quotes: &[String], min_words: usize) -> String {
    if quotes.is_empty() {
        return String::new();
    }

    let mut rng = thread_rng();
    let mut result = String::new();
    let mut word_count = 0;

    while word_count < min_words {
        let quote = &quotes[rng.gen_range(0..quotes.len())];
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(quote);
        word_count += quote.split_whitespace().count();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_text_word_count() {
        let words = vec!["hello".to_string(), "world".to_string(), "test".to_string()];
        let text = generate_text(&words, 5, false);
        let word_count = text.split_whitespace().count();
        assert_eq!(word_count, 5);
    }

    #[test]
    fn test_generate_text_no_trailing_space() {
        let words = vec!["hello".to_string()];
        let text = generate_text(&words, 3, false);
        assert!(!text.ends_with(' '));
    }

    #[test]
    fn test_generate_text_punctuation_word_count() {
        let words = vec!["hello".to_string(), "world".to_string(), "test".to_string()];
        let text = generate_text(&words, 20, true);
        let word_count = text.split_whitespace().count();
        assert_eq!(word_count, 20);
    }

    #[test]
    fn test_generate_text_punctuation_has_caps_and_punct() {
        let words = vec!["hello".to_string(), "world".to_string(), "test".to_string()];
        let text = generate_text(&words, 30, true);
        assert!(text.contains('.') || text.contains('!') || text.contains('?'));
        assert!(text.chars().next().map(|c| c.is_uppercase()).unwrap_or(false));
    }

    #[test]
    fn test_generate_quote_text_meets_min_words() {
        let quotes = vec![
            "The quick brown fox jumps over the lazy dog.".to_string(),
            "Hello world.".to_string(),
        ];
        let text = generate_quote_text(&quotes, 10);
        let word_count = text.split_whitespace().count();
        assert!(word_count >= 10);
    }

    #[test]
    fn test_generate_quote_text_empty() {
        let text = generate_quote_text(&[], 10);
        assert_eq!(text, "");
    }

    #[test]
    fn test_generate_code_text_meets_min_words() {
        let snippets = vec![
            "fn main() {\n    println!(\"hello\");\n}".to_string(),
            "let x = 42;".to_string(),
        ];
        let text = generate_code_text(&snippets, 10);
        let word_count = text.split_whitespace().count();
        assert!(word_count >= 10);
    }

    #[test]
    fn test_generate_code_text_empty() {
        let text = generate_code_text(&[], 10);
        assert_eq!(text, "");
    }

    #[test]
    fn test_generate_code_text_has_newlines() {
        let snippets = vec!["fn main() {\n    println!(\"hi\");\n}".to_string()];
        let text = generate_code_text(&snippets, 5);
        assert!(text.contains('\n'));
    }

    #[test]
    fn test_generate_practice_text_no_stats() {
        let words = vec!["hello".to_string(), "world".to_string(), "test".to_string()];
        let stats = HashMap::new();
        let text = generate_practice_text(&words, &stats, 5);
        let word_count = text.split_whitespace().count();
        assert_eq!(word_count, 5);
    }

    #[test]
    fn test_generate_practice_text_with_stats() {
        let words = vec!["hello".to_string(), "world".to_string(), "test".to_string()];
        let mut stats = HashMap::new();
        stats.insert(
            'x',
            KeyStat {
                total_time_ms: 500,
                press_count: 5,
                error_count: 3,
            },
        );
        let text = generate_practice_text(&words, &stats, 5);
        let word_count = text.split_whitespace().count();
        assert_eq!(word_count, 5);
    }
}
