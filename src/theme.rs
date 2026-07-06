use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Theme {
    #[default]
    Dark,
    Light,
    Retro,
}

impl Theme {
    pub fn label(&self) -> &'static str {
        match self {
            Theme::Dark => "Dark",
            Theme::Light => "Light",
            Theme::Retro => "Retro",
        }
    }

    pub fn title(&self) -> Color {
        match self {
            Theme::Dark => Color::Cyan,
            Theme::Light => Color::Blue,
            Theme::Retro => Color::Yellow,
        }
    }

    pub fn correct(&self) -> Color {
        match self {
            Theme::Dark => Color::Green,
            Theme::Light => Color::Green,
            Theme::Retro => Color::Green,
        }
    }

    pub fn wrong(&self) -> Color {
        match self {
            Theme::Dark => Color::Red,
            Theme::Light => Color::Red,
            Theme::Retro => Color::Red,
        }
    }

    pub fn cursor(&self) -> Color {
        match self {
            Theme::Dark => Color::Yellow,
            Theme::Light => Color::Blue,
            Theme::Retro => Color::White,
        }
    }

    pub fn untyped(&self) -> Color {
        match self {
            Theme::Dark => Color::DarkGray,
            Theme::Light => Color::Gray,
            Theme::Retro => Color::DarkGray,
        }
    }

    pub fn time(&self) -> Color {
        match self {
            Theme::Dark => Color::Cyan,
            Theme::Light => Color::Blue,
            Theme::Retro => Color::Yellow,
        }
    }

    pub fn wpm(&self) -> Color {
        match self {
            Theme::Dark => Color::Green,
            Theme::Light => Color::Green,
            Theme::Retro => Color::Green,
        }
    }

    pub fn accuracy(&self) -> Color {
        match self {
            Theme::Dark => Color::Yellow,
            Theme::Light => Color::Blue,
            Theme::Retro => Color::Cyan,
        }
    }

    pub fn help(&self) -> Color {
        match self {
            Theme::Dark => Color::DarkGray,
            Theme::Light => Color::Gray,
            Theme::Retro => Color::DarkGray,
        }
    }

    pub fn best(&self) -> Color {
        match self {
            Theme::Dark => Color::Magenta,
            Theme::Light => Color::Magenta,
            Theme::Retro => Color::Magenta,
        }
    }

    pub fn selected(&self) -> Color {
        match self {
            Theme::Dark => Color::Yellow,
            Theme::Light => Color::Blue,
            Theme::Retro => Color::White,
        }
    }

    pub fn current(&self) -> Color {
        match self {
            Theme::Dark => Color::Green,
            Theme::Light => Color::Green,
            Theme::Retro => Color::Green,
        }
    }

    pub fn unselected(&self) -> Color {
        match self {
            Theme::Dark => Color::DarkGray,
            Theme::Light => Color::Gray,
            Theme::Retro => Color::DarkGray,
        }
    }
}
