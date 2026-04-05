const HEIGHT: usize = 5;

const DIGIT_0: [&str; HEIGHT] = [
    " ████ ",
    "██  ██",
    "██  ██",
    "██  ██",
    " ████ ",
];

const DIGIT_1: [&str; HEIGHT] = [
    "  ██  ",
    " ███  ",
    "  ██  ",
    "  ██  ",
    " ████ ",
];

const DIGIT_2: [&str; HEIGHT] = [
    " ████ ",
    "██  ██",
    "  ██  ",
    " ██   ",
    "██████",
];

const DIGIT_3: [&str; HEIGHT] = [
    " ████ ",
    "██  ██",
    "   ██ ",
    "██  ██",
    " ████ ",
];

const DIGIT_4: [&str; HEIGHT] = [
    "██  ██",
    "██  ██",
    "██████",
    "    ██",
    "    ██",
];

const DIGIT_5: [&str; HEIGHT] = [
    "██████",
    "██    ",
    "█████ ",
    "    ██",
    "█████ ",
];

const DIGIT_6: [&str; HEIGHT] = [
    " ████ ",
    "██    ",
    "█████ ",
    "██  ██",
    " ████ ",
];

const DIGIT_7: [&str; HEIGHT] = [
    "██████",
    "    ██",
    "   ██ ",
    "  ██  ",
    "  ██  ",
];

const DIGIT_8: [&str; HEIGHT] = [
    " ████ ",
    "██  ██",
    " ████ ",
    "██  ██",
    " ████ ",
];

const DIGIT_9: [&str; HEIGHT] = [
    " ████ ",
    "██  ██",
    " █████",
    "    ██",
    " ████ ",
];

const DIGITS: [[&str; HEIGHT]; 10] = [
    DIGIT_0, DIGIT_1, DIGIT_2, DIGIT_3, DIGIT_4,
    DIGIT_5, DIGIT_6, DIGIT_7, DIGIT_8, DIGIT_9,
];

pub const FONT_HEIGHT: usize = HEIGHT;
pub const DIGIT_WIDTH: usize = 6;

/// Render a number as ASCII art, returning one string per line
pub fn render_number(n: u32) -> Vec<String> {
    let digits: Vec<usize> = if n == 0 {
        vec![0]
    } else {
        n.to_string()
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as usize))
            .collect()
    };

    (0..HEIGHT)
        .map(|row| {
            digits
                .iter()
                .map(|&d| DIGITS[d][row])
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect()
}

/// Calculate total width needed for a number
pub fn number_width(n: u32) -> usize {
    let digit_count = if n == 0 { 1 } else { (n as f64).log10().floor() as usize + 1 };
    digit_count * DIGIT_WIDTH + (digit_count.saturating_sub(1)) // digits + spaces between
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_single_digit() {
        let lines = render_number(5);
        assert_eq!(lines.len(), HEIGHT);
        assert_eq!(lines[0], "██████");
    }

    #[test]
    fn test_render_multi_digit() {
        let lines = render_number(12);
        assert_eq!(lines.len(), HEIGHT);
        // Should have two digits with space between
        assert!(lines[0].contains(" "));
    }

    #[test]
    fn test_number_width() {
        assert_eq!(number_width(5), 6);      // single digit
        assert_eq!(number_width(12), 13);    // two digits + space
        assert_eq!(number_width(125), 20);   // three digits + 2 spaces
    }
}
