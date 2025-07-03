use std::{io};

use ratatui::{
    crossterm::event::{
        self, KeyCode, KeyEvent
    }, symbols::border, widgets::{Block, Paragraph}, DefaultTerminal, Frame
};
pub struct App {
    running: bool
}

impl App {
    pub fn new() -> Self {
        App { running: true }
    }

    pub fn run(&mut self, term: &mut DefaultTerminal) -> io::Result<()> {
        while self.running {
            term.draw(|frame: &mut Frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
       let b = Block::bordered().border_set(border::THICK);
       let p = Paragraph::new("Hello, world!")
            .block(b)
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(p, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            event::Event::Key(key_event) => self.handle_key_event(key_event),
            event::Event::Resize(_, _) => (), // Handle resize if needed
            _ => (), // Ignore other events
        } 
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Esc => self.exit(),
            _ => () 
        }
    }

    fn exit(&mut self) {
        self.running = false;
    }
}