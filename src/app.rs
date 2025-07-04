use std::{
    fs,
    io, 
};
use ratatui::{
    crossterm::event::{ self, KeyCode, KeyEvent }, layout::Rect, DefaultTerminal, Frame
};
use rodio::{
    OutputStream, 
    OutputStreamHandle
};
use crate::components::{sound_item::SoundItem, sounds_block::SoundsBlock};
use crate::config::Config;

const RESOURCES_PATH: &str = "resources/";
const DEFAULT_VOLUME: f32 = 0.5;

pub struct App{
    running: bool,
    sounds_block: SoundsBlock,
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
        let sounds_block = SoundsBlock::default();
        App { running: true, sounds_block, stream_handle, _stream: stream, general_play_state: true }
    }

    pub fn run(&mut self, term: &mut DefaultTerminal) -> io::Result<()> {
        self.setup_list();
        while self.running {
            let size = term.size().unwrap();
            self.sounds_block.handle_resize(Rect::new(0, 0, size.width.into(), size.height.into()));
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
                self.sounds_block.add_sound(sound_item);
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(&self.sounds_block, frame.area());
        return;
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            event::Event::Key(key_event) => self.handle_key_event(key_event),
            event::Event::Resize(c, r) => {self.sounds_block.handle_resize(Rect::new(0, 0, c, r))}, // Handle resize if needed
            _ => (), // Ignore other events
        } 
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Esc => self.exit(),
            _ => {
                self.sounds_block.handle_key_event(key_event.code); 
            }
        }
    }

    fn exit(&mut self) {
        self.running = false;
    }
}