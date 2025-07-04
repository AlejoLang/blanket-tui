use std::{
    fs,
    io, 
};
use ratatui::{
    crossterm::event::{ self, KeyCode, KeyEvent }, 
    layout::{ Constraint, Layout }, 
    DefaultTerminal, 
    Frame
};
use rodio::{
    OutputStream, 
    OutputStreamHandle
};
use crate::components::{sound_item::SoundItem};
use crate::config::Config;

const RESOURCES_PATH: &str = "resources/";
const DEFAULT_VOLUME: f32 = 0.5;

pub struct App{
    running: bool,
    sounds_list: Vec<SoundItem>,
    stream_handle: Option<OutputStreamHandle>,
    _stream: Option<OutputStream>,
    general_play_state: bool,
}

impl App {
    pub fn default() -> Self {
        let (stream, stream_handle) = match OutputStream::try_default() {
            Ok((s, h)) => (Some(s), Some(h)),
            Err(e) => {
                eprintln!("Warning: No audio device available: {}. Audio functionality will be disabled.", e);
                (None, None)
            }
        };
        App { running: true, sounds_list: vec![], stream_handle, _stream: stream, general_play_state: true }
    }

    pub fn run(&mut self, term: &mut DefaultTerminal) -> io::Result<()> {
        self.setup_list();
        while self.running {
            term.draw(|frame: &mut Frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn setup_list(&mut self) {
        let sounds_file = fs::read_to_string(RESOURCES_PATH.to_string() + "sounds.toml");
        
        if sounds_file.is_err() {
            eprintln!("Warning: sounds.toml file not found. No sounds will be loaded.");
        } else {
            let toml_file = sounds_file.unwrap();
            let config: Config = toml::from_str(&toml_file).unwrap();
            for (i, sound) in config.sound.iter().enumerate() {
                let sound_item = SoundItem::new(
                    i as u32, 
                    sound.name.clone(), 
                    RESOURCES_PATH.to_string() + &sound.file, 
                    DEFAULT_VOLUME,
                    sound.icon.clone(), 
                    i == 0, 
                    false, 
                    self.stream_handle.as_ref()
                );
                self.sounds_list.push(sound_item);
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        if self.sounds_list.is_empty() {
            // Display a message when no sounds are available
            let paragraph = ratatui::widgets::Paragraph::new("No sound files found in the sounds directory")
                .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow))
                .alignment(ratatui::layout::Alignment::Center);
            frame.render_widget(paragraph, frame.area());
            return;
        }
        
        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![Constraint::Percentage(100 / self.sounds_list.len() as u16); self.sounds_list.len()])
            .split(frame.area());
        for (i, sound_item) in self.sounds_list.iter().enumerate() {
            frame.render_widget(sound_item, chunks[i]);
        }
    }

    fn get_selected_sound_mut(&mut self) -> Option<&mut SoundItem> {
        self.sounds_list.iter_mut().find(|item| item.is_selected())
    }

    pub fn get_general_play_state(&mut self) -> bool {
        self.general_play_state
    }

    fn select_previous_sound(&mut self) {
        for (i, sound) in self.sounds_list.iter_mut().enumerate() {
            if sound.is_selected() {
                sound.toggle_selection();
                let previous_index = if i == 0 { self.sounds_list.len() - 1 } else { i - 1 };
                self.sounds_list[previous_index].toggle_selection();
                return;
            }
        } 
    }

    fn select_next_sound(&mut self) {
        for (i, sound) in self.sounds_list.iter_mut().enumerate() {
            if sound.is_selected() {
                sound.toggle_selection();
                let previous_index = if i == self.sounds_list.len() - 1 { 0 } else { i + 1 };
                self.sounds_list[previous_index].toggle_selection();
                return;
            }
        } 
    }

    fn switch_play_pause_all(&mut self) {
        self.general_play_state = !self.general_play_state;
        for sound_item in &mut self.sounds_list {
            if sound_item.is_active() {
                sound_item.switch_play_pause();
            } 
        }
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
            KeyCode::Up => { self.select_previous_sound(); }
            KeyCode::Down => { self.select_next_sound(); }
            KeyCode::Enter => { self.switch_play_pause_all(); }
            _ => {
                if let Some(selected_sound) = self.get_selected_sound_mut() {
                    if let Err(e) = selected_sound.handle_key_event(key_event.code) {
                        eprintln!("Error handling key event: {}", e);
                    }
                } 
            }
        }
    }

    fn exit(&mut self) {
        self.running = false;
    }
}