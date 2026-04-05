use std::time::Duration;

/// Calculate WPM using the standard formula: (characters / 5) / minutes
/// A "word" is standardized as 5 characters for WPM calculations
pub fn calculate_wpm(chars_typed: usize, duration: Duration) -> f64 {
    let minutes = duration.as_secs_f64() / 60.0;
    if minutes == 0.0 {
        return 0.0;
    }

    let words = chars_typed as f64 / 5.0;
    words / minutes
}

/// Calculate accuracy as percentage of correct keystrokes
pub fn calculate_accuracy(correct: usize, total: usize) -> f64 {
    if total == 0 {
        return 100.0;
    }
    (correct as f64 / total as f64) * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wpm_calculation() {
        // 50 characters in 1 minute = 10 WPM (50/5 = 10 words)
        let wpm = calculate_wpm(50, Duration::from_secs(60));
        assert!((wpm - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_wpm_zero_duration() {
        let wpm = calculate_wpm(50, Duration::ZERO);
        assert_eq!(wpm, 0.0);
    }

    #[test]
    fn test_accuracy_perfect() {
        let acc = calculate_accuracy(100, 100);
        assert!((acc - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_accuracy_half() {
        let acc = calculate_accuracy(50, 100);
        assert!((acc - 50.0).abs() < 0.001);
    }

    #[test]
    fn test_accuracy_no_input() {
        let acc = calculate_accuracy(0, 0);
        assert_eq!(acc, 100.0);
    }
}
