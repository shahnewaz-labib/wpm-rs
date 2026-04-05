use color_eyre::Result;
use rand::prelude::*;
use serde_json::Value;
use std::fs;

const DEFAULT_WORDS: &str = include_str!("../quotes/english.json");

pub fn load_words(path: &str) -> Result<Vec<String>> {
    let content = fs::read_to_string(path).unwrap_or_else(|_| DEFAULT_WORDS.to_string());
    let json: Value = serde_json::from_str(&content)?;

    let words = json["words"]
        .as_array()
        .ok_or_else(|| color_eyre::eyre::eyre!("Invalid JSON: missing 'words' array"))?
        .iter()
        .filter_map(|x| x.as_str().map(String::from))
        .collect();

    Ok(words)
}

pub fn generate_text(words: &[String], count: usize) -> String {
    let mut rng = thread_rng();
    let mut result = String::new();

    for i in 0..count {
        let index = rng.gen_range(0..words.len());
        result.push_str(&words[index]);
        if i < count - 1 {
            result.push(' ');
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_text_word_count() {
        let words = vec!["hello".to_string(), "world".to_string(), "test".to_string()];
        let text = generate_text(&words, 5);
        let word_count = text.split_whitespace().count();
        assert_eq!(word_count, 5);
    }

    #[test]
    fn test_generate_text_no_trailing_space() {
        let words = vec!["hello".to_string()];
        let text = generate_text(&words, 3);
        assert!(!text.ends_with(' '));
    }
}
