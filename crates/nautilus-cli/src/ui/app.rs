use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use std::io::stdout;
use tokio::time::{interval, Duration};
use futures_util::StreamExt;
use anyhow::Result;

pub struct App {
    should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self { should_quit: false }
    }

    pub async fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        // Render loop ticker
        let mut tick_rate = interval(Duration::from_millis(16)); // ~60 FPS

        // Async event stream
        let mut events = crossterm::event::EventStream::new();

        while !self.should_quit {
            tokio::select! {
                _ = tick_rate.tick() => {
                    terminal.draw(|f| self.draw(f))?;
                }
                Some(Ok(event)) = events.next() => {
                    self.handle_event(event);
                }
            }
        }

        // Teardown terminal
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    fn draw(&self, f: &mut Frame) {
        let size = f.size();
        let block = Block::default()
            .title("Nautilus Deck")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));
            
        let text = Paragraph::new("Press 'q' or Ctrl+C to quit.")
            .block(block)
            .alignment(Alignment::Center);

        f.render_widget(text, size);
    }

    fn handle_event(&mut self, event: Event) {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => self.should_quit = true,
                    KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => self.should_quit = true,
                    _ => {}
                }
            }
        }
    }
}
