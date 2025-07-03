use std::{fs::File, io::{BufReader}};
use rodio::{Decoder, OutputStreamHandle, Sink, Source};

pub struct Sound {
    path: String,
    volume: f32,
    sink: Option<Sink>,
}

impl Sound {
    pub fn new(path: String, volume: f32, stream_handle: &OutputStreamHandle) -> Self {
        let sink = Sink::try_new(&stream_handle).ok();
        if let Some(ref sink) = sink {
            if let Ok(file) = File::open(&path) {
                let file = BufReader::new(file);
                if let Ok(source) = Decoder::new(file) {
                    sink.append(source.repeat_infinite());
                    sink.pause();
                }
            }
        }
        Sound { path, volume, sink }
    }

    pub fn new_no_audio(path: String, volume: f32) -> Self {
        Sound { path, volume, sink: None }
    }

    pub fn play(&self) {
        if let Some(ref sink) = self.sink {
            sink.play();
        }
    }

    pub fn pause(&self) {
        if let Some(ref sink) = self.sink {
            sink.pause();
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
        if let Some(ref sink) = self.sink {
            sink.set_volume(self.volume);
        }
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }
}