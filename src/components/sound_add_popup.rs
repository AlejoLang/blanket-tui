use std::fs;

use ratatui::{buffer::Buffer, crossterm::event::{KeyCode, KeyEvent}, layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{self, Color, Style, Stylize}, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph, Widget}};
use crate::{app::RESOURCES_PATH, components::input::Input};
use crate::config::{Config, SoundConfig};

pub struct SoundAddPopup {
    inputs: Vec<Input>,
    opened: bool,
}

impl SoundAddPopup {
    pub fn new() -> Self {
        let name_input = Input::new(" Name ".to_string(), 50, true);
        let file_input = Input::new(" File Path ".to_string(), 100, false);
        let icon_input = Input::new(" Icon ".to_string(), 1, false);
        SoundAddPopup {
            inputs: vec![name_input, file_input, icon_input],
            opened: false,
        }
    }

    pub fn get_name(&self) -> &str {
        self.inputs[0].get_text()
    }

    pub fn get_file_path(&self) -> &str {
        self.inputs[1].get_text()
    }

    pub fn get_icon_path(&self) -> &str {
        self.inputs[2].get_text()
    }

    pub fn get_opened(&self) -> bool {
        self.opened
    }

    pub fn clear(&mut self) {
        for input in &mut self.inputs {
            input.set_active(false);
            input.clear();
        }
        self.inputs[0].set_active(true); // Activate the first input
    }

    pub fn set_opened(&mut self, opened: bool) {
        self.opened = opened;
    }

    pub fn cycle_active_input(&mut self) {
        for (i, input) in &mut self.inputs.iter_mut().enumerate() {
            if input.get_active() {
                input.switch_active();
                let next_index = (i + 1) % self.inputs.len();
                self.inputs[next_index].switch_active();
                break;
            }
        }
    }

    pub fn submit_instruction(&mut self) {
        for input in &mut self.inputs {
            if input.get_text().is_empty() {
                input.switch_active();
                return;
            }
        }
        let sounds_file = fs::read_to_string(RESOURCES_PATH.to_string() + "sounds.toml");
        if let Err(e) = sounds_file {
            eprintln!("Error reading sounds file: {}", e);
            return;
        }
        let toml_file = sounds_file.unwrap();
        let mut config: Config = toml::from_str(&toml_file).unwrap();
        config.sound.push(SoundConfig {
            name: self.get_name().to_string(),
            file: self.get_file_path().to_string(),
            icon: self.get_icon_path().to_string(),
        });
        let new_toml = toml::to_string(&config).unwrap();
        if let Err(e) = fs::write(RESOURCES_PATH.to_string() + "sounds.toml", new_toml) {
            eprintln!("Error writing to sounds file: {}", e);
            return;
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Tab => {
                self.cycle_active_input();
            }
            KeyCode::Enter => {
                self.submit_instruction();
                self.set_opened(false);
                self.clear();
            }
            _ => {
                for input in &mut self.inputs {
                    if input.get_active() {
                        input.handle_key_event(key_event);
                        break;
                    }
                }
            }
        } 
    }
}

impl Widget for &SoundAddPopup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        
        let div_vert = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Min(14), Constraint::Fill(1)])
            .split(area);
        let div_vert_hor = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Fill(1), Constraint::Min(30), Constraint::Fill(1)])
            .split(div_vert[1]);

        let block = Block::bordered()
            .title(" Add Sound ".bold())
            .title_alignment(Alignment::Center)
            .border_set(border::EMPTY)
            .style(Style::default().bg(Color::Black));
        block.render(div_vert_hor[1], buf);
        let mut constraints = vec![Constraint::Length(3); self.inputs.len()];
        constraints.insert(0, Constraint::Max(2));
        constraints.push(Constraint::Length(1));
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(constraints)
            .split(div_vert_hor[1]);

        let instructions_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Fill(1); 2])
            .split(chunks[0]);

        let quit_insruction = Line::from(vec![
            "[Esc]".bold(),
            " Close".into()
        ]).alignment(Alignment::Left);
        let switch_instruction = Line::from(vec![
            "[Tab]".bold(),
            " Switch".into()
        ]).alignment(Alignment::Right);
        let submit_instruction = Line::from(vec![
            "[Enter]".bold(),
            " Submit".into()
        ]).alignment(Alignment::Center);

        quit_insruction.render(instructions_chunks[0], buf);
        switch_instruction.render(instructions_chunks[1], buf);
        submit_instruction.render(chunks[chunks.len()-1], buf);

        for (i, input) in self.inputs.iter().enumerate() {
            let input_area = chunks[i + 1];
            input.render(input_area, buf);
        }
    }
}