use ratatui::{crossterm::event::{KeyCode, KeyEvent}, style::{Color, Style, Stylize}, symbols::border, text::Text, widgets::{Block, Paragraph, Widget}};


pub struct Input {
    name: String,
    max_length: usize,
    active: bool,
    text: String,
    character_index: usize,
}

impl Input {
    pub fn new(name: String, max_length: usize, active: bool) -> Self {
        Input {
            name,
            max_length,
            active,
            text: String::new(),
            character_index: 0,
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_character_index(&mut self, position: usize) {
        self.character_index = position.clamp(0, self.text.len());
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn switch_active(&mut self) {
        self.active = !self.active;
    }

    pub fn get_active(&self) -> bool {
        self.active
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.character_index = 0;
    }

    fn move_character_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.set_character_index(cursor_moved_left);
    }

    fn move_character_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.set_character_index(cursor_moved_right);
    }

    fn enter_character(&mut self, character: char) {
        if character == '\n' || self.text.len() >= self.max_length {
            return; // Ignore newline characters
        }
        self.text.insert(self.character_index, character);
        self.move_character_right();
    }

    fn delete_character(&mut self) {
        if self.character_index > 0 {
            self.text.remove(self.character_index - 1);
            self.move_character_left();
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Left => self.move_character_left(),
            KeyCode::Right => self.move_character_right(),
            KeyCode::Backspace => self.delete_character(),
            KeyCode::Char(c) => self.enter_character(c),
            _ => {}
        }
    }
}

impl Widget for &Input {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        let min = self.character_index.saturating_sub(area.width as usize - 2).clamp(0, self.text.len());
        let max = (self.character_index).clamp(0, self.text.len());

        let name = self.name.clone();
        let text = Text::from(self.get_text()[min..max].to_string())
            .style(Style::default().fg(Color::White));
        let block = Block::bordered()
            .title(name)
            .border_set(border::ROUNDED)
            .border_style({
                if self.active {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::White)
                }
            });
        let paragraph = Paragraph::new(text)
            .block(block)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .scroll((0, 0))
            .alignment(ratatui::layout::Alignment::Left); 
        paragraph.render(area, buf);
    }
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.max_length == other.max_length && self.active == other.active && self.text == other.text
    }
}