// src/pages/hints.rs
//
// Keyboard hint styling system - btop-style hints with highlighted trigger keys.
// Makes it easy to change colors globally and create consistent hotkey hints.

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

// ============================================================================
// CONFIGURABLE COLORS - Change these to update hint styling globally
// ============================================================================

/// Color for the highlighted trigger key (the underlined letter)
pub const HINT_KEY_COLOR: Color = Color::Yellow;

/// Color for the rest of the hint text (the action description)
pub const HINT_TEXT_COLOR: Color = Color::DarkGray;

/// Color for separators between hints
pub const HINT_SEPARATOR_COLOR: Color = Color::DarkGray;

/// Modifier for the trigger key (UNDERLINED makes it look like btop)
pub const HINT_KEY_MODIFIER: Modifier = Modifier::UNDERLINED;

// ============================================================================
// HINT BUILDING UTILITIES
// ============================================================================

/// Creates a styled hint where the first character is highlighted.
/// Example: `hint("quit")` produces "**q**uit" with 'q' underlined and yellow.
pub fn hint(text: &str) -> Vec<Span<'static>> {
    if text.is_empty() {
        return vec![Span::raw("")];
    }

    let first_char = text.chars().next().unwrap();
    let rest = &text[first_char.len_utf8()..];

    vec![
        Span::styled(
            first_char.to_string(),
            Style::default()
                .fg(HINT_KEY_COLOR)
                .add_modifier(HINT_KEY_MODIFIER),
        ),
        Span::styled(rest.to_string(), Style::default().fg(HINT_TEXT_COLOR)),
    ]
}

/// Creates a styled hint with a custom key and action.
/// Example: `hint_key("Enter", "chat")` produces "**E**nter:chat"
pub fn hint_key(key: &str, action: &str) -> Vec<Span<'static>> {
    if key.is_empty() {
        return vec![Span::raw("")];
    }

    let first_char = key.chars().next().unwrap();
    let rest = &key[first_char.len_utf8()..];

    vec![
        Span::styled(
            first_char.to_string(),
            Style::default()
                .fg(HINT_KEY_COLOR)
                .add_modifier(HINT_KEY_MODIFIER),
        ),
        Span::styled(
            format!("{}:{}", rest, action),
            Style::default().fg(HINT_TEXT_COLOR),
        ),
    ]
}

/// Creates a separator span for between hints
pub fn separator() -> Span<'static> {
    Span::styled("  ", Style::default().fg(HINT_SEPARATOR_COLOR))
}

/// Creates a pipe separator for between hints
pub fn pipe_separator() -> Span<'static> {
    Span::styled(" | ", Style::default().fg(HINT_SEPARATOR_COLOR))
}

/// Builds a Line from multiple hints with separators.
/// Example: `hints_line(&[hint("quit"), hint_key("Enter", "chat")])`
pub fn hints_line(hints: &[Vec<Span<'static>>]) -> Line<'static> {
    let mut spans: Vec<Span<'static>> = Vec::new();

    for (i, hint_spans) in hints.iter().enumerate() {
        if i > 0 {
            spans.push(separator());
        }
        spans.extend(hint_spans.clone());
    }

    Line::from(spans)
}

/// Builds a Line from multiple hints with pipe separators.
pub fn hints_line_piped(hints: &[Vec<Span<'static>>]) -> Line<'static> {
    let mut spans: Vec<Span<'static>> = Vec::new();

    for (i, hint_spans) in hints.iter().enumerate() {
        if i > 0 {
            spans.push(pipe_separator());
        }
        spans.extend(hint_spans.clone());
    }

    Line::from(spans)
}

// ============================================================================
// ADDITIONAL HELPERS
// ============================================================================

/// Creates a plain text span (not a hint, just text)
pub fn plain(text: &str) -> Vec<Span<'static>> {
    vec![Span::styled(
        text.to_string(),
        Style::default().fg(HINT_TEXT_COLOR),
    )]
}

/// Creates an info span with custom color
pub fn info(text: &str, color: Color) -> Vec<Span<'static>> {
    vec![Span::styled(text.to_string(), Style::default().fg(color))]
}

/// Creates a status span (for online/offline, player count, etc.)
pub fn status(text: &str) -> Vec<Span<'static>> {
    vec![Span::styled(
        text.to_string(),
        Style::default().fg(Color::White),
    )]
}

/// Arrow key hint (special case since arrows aren't typable)
pub fn arrows(action: &str) -> Vec<Span<'static>> {
    vec![
        Span::styled(
            "↑↓←→",
            Style::default()
                .fg(HINT_KEY_COLOR)
                .add_modifier(HINT_KEY_MODIFIER),
        ),
        Span::styled(format!(":{}", action), Style::default().fg(HINT_TEXT_COLOR)),
    ]
}
