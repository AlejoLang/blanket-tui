use std::io;

use ratatui::{
    buffer::Buffer, 
    layout::{Alignment, Constraint, Layout, Rect}, 
    style::{Color, Style, Stylize}, 
    text::Text, 
    widgets::{Paragraph, Widget},
    crossterm::event::KeyCode,
};
use rodio::OutputStreamHandle;
use crate::components::sound::Sound;

pub struct SoundItem {
    id: u32,
    name: String,
    icon: String,
    selected: bool,
    sound: Sound
}

impl SoundItem {
    pub fn new(id: u32, name: String, path: String, volume: f32, icon: String, selected: bool, stream_handle: Option<&OutputStreamHandle>) -> Self {
        let sound = match stream_handle {
            Some(handle) => Sound::new(path, volume, handle),
            None => {
                // Create a sound that will have None sink (audio disabled)
                Sound::new_no_audio(path, volume)
            }
        };
        SoundItem {
            id,
            name,
            icon,
            selected,
            sound
        }
    }

    pub fn toggle_selection(&mut self) {
        self.selected = !self.selected;
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn is_playing(&self) -> bool {
        return self.sound.is_playing();
    }

    pub fn get_icon(&self) -> &str {
        &self.icon
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
    
    pub fn change_volume(&mut self, delta: f32) {
        self.sound.set_volume(self.sound.get_volume() + delta);
    }

    pub fn switch_play_pause(&mut self) {
        self.sound.switch_play_pause();
    }

    pub fn handle_key_event(&mut self, key: KeyCode) -> io::Result<()> {
        if self.selected {
            match key {
                KeyCode::Left => { self.change_volume(-0.05); },
                KeyCode::Right => { self.change_volume(0.05); },
                KeyCode::Char(' ') => { self.sound.switch_play_pause(); }
                _ => {}
            }
        }
        Ok(())
    }
}

impl Widget for &SoundItem {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Crear el layout horizontal para dividir el área en dos columnas
        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                Constraint::Min(1),    // Nombre - toma el espacio restante
                Constraint::Length(15) // Volumen - ancho fijo de 15 caracteres
            ])
            .split(area);

        // Crear el texto del nombre (lado izquierdo)
        let name_style = if self.selected {
            Style::default().fg(Color::Yellow).bold()
        } else {
            Style::default().fg(Color::White)
        };
        
        let mut name_paragraph = Paragraph::new(Text::from(format!("{} {}", self.icon, self.name)))
            .style(name_style)
            .alignment(Alignment::Left);

        // Crear el texto del volumen (lado derecho)
        let volume_text = format!("Vol: {:.0}%", self.sound.get_volume() * 100.0);
        let mut volume_paragraph = Paragraph::new(Text::from(volume_text))
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Right);
        if self.is_playing() {
            // Si el sonido está reproduciendo, resaltar el fondo
            name_paragraph = name_paragraph.bg(Color::Green);
            volume_paragraph = volume_paragraph.bg(Color::Green);
        }
        if self.selected {
            // Si el elemento está seleccionado, resaltar el fondo
            name_paragraph = name_paragraph.bg(Color::Blue);
            volume_paragraph = volume_paragraph.bg(Color::Blue);
        }
        // Renderizar ambos componentes
        name_paragraph.render(chunks[0], buf);
        volume_paragraph.render(chunks[1], buf);
    }
}