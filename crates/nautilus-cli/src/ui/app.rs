use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, List, ListItem};
use std::io::stdout;
use tokio::time::{interval, Duration};
use futures_util::StreamExt;
use anyhow::Result;
use tokio::sync::mpsc;

pub struct App {
    should_quit: bool,
    logs: Vec<String>,
    log_receiver: mpsc::Receiver<String>,
}

impl App {
    pub fn new(log_receiver: mpsc::Receiver<String>) -> Self {
        Self { 
            should_quit: false,
            logs: Vec::new(),
            log_receiver,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        let mut tick_rate = interval(Duration::from_millis(16)); // ~60 FPS
        let mut events = crossterm::event::EventStream::new();

        while !self.should_quit {
            tokio::select! {
                _ = tick_rate.tick() => {
                    terminal.draw(|f| self.draw(f))?;
                }
                Some(Ok(event)) = events.next() => {
                    self.handle_event(event);
                }
                Some(log_line) = self.log_receiver.recv() => {
                    self.logs.push(log_line);
                    // Keep buffer bounded
                    if self.logs.len() > 1000 {
                        self.logs.remove(0);
                    }
                }
            }
        }

        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    fn draw(&self, f: &mut Frame) {
        let size = f.size();
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(10),   // Main
                Constraint::Length(3), // Footer
            ])
            .split(size);

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30), // DAG
                Constraint::Percentage(70), // Logs
            ])
            .split(chunks[1]);

        // Header
        let header = Paragraph::new("Nautilus Execution Engine")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // DAG Widget
        // TODO: Map actual DAG nodes
        let items = [
            ListItem::new(" ⏳ build-image ").style(Style::default().fg(Color::DarkGray)),
            ListItem::new(" 🌀 run-tests ").style(Style::default().fg(Color::Cyan)),
            ListItem::new(" ✅ lint ").style(Style::default().fg(Color::Green)),
            ListItem::new(" ❌ deploy ").style(Style::default().fg(Color::Red)),
        ];
        let dag_list = List::new(items)
            .block(Block::default().title(" Pipeline DAG ").borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
        f.render_widget(dag_list, main_chunks[0]);

        // Logs Widget
        let log_items: Vec<ListItem> = self.logs
            .iter()
            .map(|l| ListItem::new(l.as_str()))
            .collect();
        let log_list = List::new(log_items)
            .block(Block::default().title(" Live Logs ").borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
        f.render_widget(log_list, main_chunks[1]);

        // Footer
        let footer = Paragraph::new("Press 'q' or Ctrl+C to quit.")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(footer, chunks[2]);
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
