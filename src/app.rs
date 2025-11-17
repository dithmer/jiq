use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::io;
use tui_textarea::TextArea;

/// Application state
pub struct App {
    json_input: String,
    textarea: TextArea<'static>,
    should_quit: bool,
}

impl App {
    /// Create a new App instance with JSON input
    pub fn new(json_input: String) -> Self {
        // Create textarea for query input
        let mut textarea = TextArea::default();

        // Configure for single-line input
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Query ")
                .border_style(Style::default().fg(Color::DarkGray)),
        );

        // Remove default underline from cursor line
        textarea.set_cursor_line_style(Style::default());

        Self {
            json_input,
            textarea,
            should_quit: false,
        }
    }

    /// Check if the application should quit
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Handle events and update application state
    pub fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // Check that it's a key press event to avoid duplicates
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        }
        Ok(())
    }

    /// Handle key press events
    fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.should_quit = true;
            }
            KeyCode::Enter => {
                // Prevent newlines in single-line input
                // Query executes on every keystroke (v0.5.0), not on Enter
            }
            _ => {
                // Pass all other keys to textarea for editing
                self.textarea.input(key);
            }
        }
    }

    /// Render the UI
    pub fn render(&self, frame: &mut Frame) {
        // Split the terminal into two panes: results (top) and input (bottom)
        let layout = Layout::vertical([
            Constraint::Min(3),      // Results pane takes most of the space
            Constraint::Length(3),   // Input field is fixed 3 lines
        ])
        .split(frame.area());

        let results_area = layout[0];
        let input_area = layout[1];

        // Render results pane
        self.render_results_pane(frame, results_area);

        // Render textarea input field
        frame.render_widget(&self.textarea, input_area);
    }

    /// Render the results pane (top)
    fn render_results_pane(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Results ")
            .border_style(Style::default().fg(Color::Cyan));

        let content = Paragraph::new(self.json_input.as_str())
            .block(block)
            .style(Style::default().fg(Color::White));

        frame.render_widget(content, area);
    }
}
