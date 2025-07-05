use ratatui::{buffer::Buffer, crossterm::event::KeyCode, layout::{Constraint, Layout, Rect}, style::Stylize, symbols::border, text::{Line, Text}, widgets::{Block, Widget}};
use crate::{app::RESOURCES_PATH, components::sound_item::SoundItem, config::Config};

pub struct SoundsBlock {
    sounds_list: Vec<SoundItem>,
    lower_bound: usize,
    upper_bound: usize,
}

impl SoundsBlock {
    pub fn new(sounds: Vec<SoundItem>) -> Self {
        SoundsBlock { sounds_list: sounds, lower_bound: 0, upper_bound: 4 }
    }

    pub fn default() -> Self {
        SoundsBlock { sounds_list: vec![], lower_bound: 0, upper_bound: 8 }
    }

    pub fn add_sound(&mut self, sound: SoundItem) {
        self.sounds_list.push(sound);
    }

    pub fn get_sounds(&self) -> &Vec<SoundItem> {
        &self.sounds_list
    }

    pub fn set_sounds(&mut self, sounds: Vec<SoundItem>) {
        self.sounds_list = sounds;
    }

    fn get_selected_sound_mut(&mut self) -> Option<(&mut SoundItem, usize)> {
        if let Some(index_of_item) = self.sounds_list.iter().position(|item| item.is_selected()) {
            return Some((&mut self.sounds_list[index_of_item], index_of_item));
        }
        None
    }

    fn select_previous_sound(&mut self) -> usize {
        for (i, sound) in self.sounds_list.iter_mut().enumerate() {
            if sound.is_selected() {
                sound.toggle_selection();
                let previous_index = if i == 0 { 0 } else { i - 1 };
                self.sounds_list[previous_index].toggle_selection();
                return previous_index;
            }
        }
        0
    }

    fn select_next_sound(&mut self) -> usize {
        for (i, sound) in self.sounds_list.iter_mut().enumerate() {
            if sound.is_selected() {
                sound.toggle_selection();
                let next_index = if i == self.sounds_list.len() - 1 { self.sounds_list.len() - 1 } else { i + 1 };
                self.sounds_list[next_index].toggle_selection();
                return next_index;
            }
        }
        0
    }

    fn switch_play_pause_all(&mut self) {
        for sound_item in &mut self.sounds_list {
            if sound_item.is_active() {
                sound_item.switch_play_pause();
            } 
        }
    }

    pub fn delete_selected_sound_from_list(&mut self) {
        let current_index = match self.sounds_list.iter().position(|item| item.is_selected()) {
            Some(index) => index,
            None => return,
        };
        let sound = self.sounds_list[current_index].clone();
        
        if current_index >= self.sounds_list.len() - 1 {
            self.select_previous_sound();
        } else {
            self.select_next_sound();
        }
        self.delete_selected_sound_from_file(&sound);
        self.sounds_list.remove(current_index);
        if current_index < self.lower_bound {
            self.lower_bound -= 1;
            self.upper_bound -= 1;
        } else if current_index <= self.upper_bound {
            self.upper_bound -= 1;
        }
    }

    fn delete_selected_sound_from_file(&mut self, sound: &SoundItem) {
        let sounds_file = std::fs::read_to_string(RESOURCES_PATH.to_string() + "sounds.toml");
        if let Err(e) = sounds_file {
            eprintln!("Error reading sounds file: {}", e);
            return;
        }
        let toml_file = sounds_file.unwrap();
        let mut config: Config = toml::from_str(&toml_file).unwrap();
        config.sound.retain(|s| s.name != sound.get_name() && s.file != sound.get_path());
        let new_toml = toml::to_string(&config).unwrap();
        std::fs::write(RESOURCES_PATH.to_string() + "sounds.toml", new_toml).unwrap();
    }

    pub fn handle_key_event(&mut self, key: KeyCode, general_play_status: bool) {
        match key {
            KeyCode::Up => {
                let i = self.select_previous_sound();
                if self.lower_bound > i {
                    self.lower_bound -= 1;
                    self.upper_bound -= 1;
                }
            }
            KeyCode::Down => {
                let i = self.select_next_sound();
                if self.upper_bound < i {
                    self.lower_bound += 1;
                    self.upper_bound += 1;
                }
            } 
            KeyCode::Enter => { self.switch_play_pause_all(); }
            KeyCode::PageUp => {
                if self.lower_bound > 0 {
                    let page_size = (self.upper_bound - self.lower_bound + 1) / 2;
                    self.lower_bound = self.lower_bound.saturating_sub(page_size);
                    self.upper_bound = (self.upper_bound - page_size).min(self.sounds_list.len() - 1);
                }
            }
            KeyCode::PageDown => {
                if self.upper_bound < self.sounds_list.len() - 1 {
                    let page_size = (self.upper_bound - self.lower_bound + 1) / 2;
                    self.lower_bound = (self.lower_bound + page_size).min(self.sounds_list.len() - 1);
                    self.upper_bound = (self.upper_bound + page_size).min(self.sounds_list.len() - 1);
                }
            }
            KeyCode::Char('d') => self.delete_selected_sound_from_list(),
            _ => {
                if let Some((selected_sound, _)) = self.get_selected_sound_mut() {
                    if let Err(e) = selected_sound.handle_key_event(key, general_play_status) {
                        eprintln!("Error handling key event: {}", e);
                    }
                } 
            }
        }
    }

    pub fn handle_resize(&mut self, area: Rect) {
        if self.sounds_list.is_empty() {
            self.lower_bound = 0;
            self.upper_bound = 0;
            return;
        }
        let selected_info = if let Some((_, index)) = self.get_selected_sound_mut() {
            Some(index)
        } else {
            None
        };
         
        let num_chunks = (area.height as usize - 4).clamp(1, self.sounds_list.len());
        self.upper_bound = (self.lower_bound + num_chunks - 1).clamp(self.lower_bound, self.sounds_list.len() - 1);
        
        if let Some(index_fixed) = selected_info {
            if self.upper_bound < index_fixed {
                self.sounds_list[index_fixed].toggle_selection();
                self.sounds_list[self.upper_bound].toggle_selection();
            }
        }
    }
}

impl Widget for &SoundsBlock {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Sounds ".bold());
        let block = Block::bordered()
            .border_set(border::THICK)
            .title(title)
            .title_alignment(ratatui::layout::Alignment::Center);
        block.render(area, buf);

        if self.sounds_list.is_empty() {
            // Display a message when no sounds are available
            let paragraph = ratatui::widgets::Paragraph::new("No sound files found in the sounds directory")
                .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow))
                .alignment(ratatui::layout::Alignment::Center);
            paragraph.render(area, buf);
            return;
        }

        let num_chunks = (area.height as usize - 4).clamp(1, self.sounds_list.len());
        let constraints = vec![Constraint::Length(1); num_chunks];
        let min = self.lower_bound.clamp(0, self.sounds_list.len() - num_chunks);
        let max = (self.lower_bound + num_chunks - 1).clamp(min, self.sounds_list.len() - 1);
        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(constraints)
            .margin(2)
            .split(area);
        for (i, sound_item) in self.sounds_list[self.lower_bound..=max].iter().enumerate() {
            sound_item.render(chunks[i], buf);
        }
    }
}
