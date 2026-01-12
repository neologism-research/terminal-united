//! Keyboard hint styling system - btop-style hints with highlighted trigger keys.

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

// =============================================================================
// CONFIGURABLE COLORS - Change these to update hint styling globally
// =============================================================================

pub const HINT_KEY_COLOR: Color = Color::Yellow;
pub const HINT_TEXT_COLOR: Color = Color::DarkGray;
pub const HINT_SEPARATOR_COLOR: Color = Color::DarkGray;
pub const HINT_KEY_MODIFIER: Modifier = Modifier::UNDERLINED;

// =============================================================================
// HINT BUILDING UTILITIES
// =============================================================================

/// Creates "**q**uit" style hint with first char highlighted
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

/// Creates "**E**nter:chat" style hint
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

fn separator() -> Span<'static> {
    Span::styled("  ", Style::default().fg(HINT_SEPARATOR_COLOR))
}

fn pipe_separator() -> Span<'static> {
    Span::styled(" | ", Style::default().fg(HINT_SEPARATOR_COLOR))
}

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

pub fn info(text: &str, color: Color) -> Vec<Span<'static>> {
    vec![Span::styled(text.to_string(), Style::default().fg(color))]
}

pub fn status(text: &str) -> Vec<Span<'static>> {
    vec![Span::styled(
        text.to_string(),
        Style::default().fg(Color::White),
    )]
}
