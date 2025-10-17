//! Dashboard logs panel component
//!
//! Renders activity logs with event formatting

use super::super::state::DashboardState;
use super::super::utils::{clean_http_error_message, format_compact_timestamp, get_worker_color};
use crate::events::EventType;
use crate::logging::LogLevel;
use ratatui::Frame;
use ratatui::prelude::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap};

/// Render enhanced logs panel with better event formatting.
pub fn render_logs_panel(f: &mut Frame, area: ratatui::layout::Rect, state: &mut DashboardState) {
    // Calculate how many log lines can fit in the available area
    // Account for borders and padding (subtract 3 for top/bottom borders + padding)
    let max_logs = (area.height.saturating_sub(3)) as usize;
    state.max_logs = max_logs;

    let displayed_logs: Vec<_> = state
        .activity_logs
        .iter()
        .filter(|event| event.should_display())
        .collect();

    let log_lines: Vec<Line> = displayed_logs
        .iter()
        .rev()
        .skip(state.log_scroll_offset)
        .take(max_logs)
        .map(|event| {
            let status_icon = match (event.event_type, event.log_level) {
                (EventType::Success, _) => "✅",
                (EventType::Error, LogLevel::Error) => "❌",
                (EventType::Error, LogLevel::Warn) => "",
                (EventType::Error, _) => "❌",
                (EventType::Refresh, _) => "",
                (EventType::Waiting, _) => "",
                (EventType::StateChange, _) => "",
            };

            let worker_color = get_worker_color(&event.worker);
            let compact_time = format_compact_timestamp(&event.timestamp);
            let cleaned_msg = clean_http_error_message(&event.msg);

            Line::from(vec![
                Span::raw(format!("{} ", status_icon)),
                Span::styled(
                    format!("{} ", compact_time),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(cleaned_msg, Style::default().fg(worker_color)),
            ])
        })
        .collect();

    let scroll_indicator = if displayed_logs.len() > max_logs {
        format!(
            " {}/{} ",
            displayed_logs.len() - state.log_scroll_offset,
            displayed_logs.len()
        )
    } else {
        "".to_string()
    };

    let title = format!("ACTIVITY LOG{}", scroll_indicator);

    let log_paragraph = if log_lines.is_empty() {
        Paragraph::new(vec![Line::from("Starting up...")])
    } else {
        Paragraph::new(log_lines)
    };

    let logs_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::uniform(1));

    let log_widget = log_paragraph.block(logs_block).wrap(Wrap { trim: true });

    f.render_widget(log_widget, area);
}
