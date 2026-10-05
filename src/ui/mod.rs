mod channel;
mod stereo_visual;
mod volume_visual;

use std::time::Duration;

use clap::Parser;
use iced::widget::{column, row};
use iced::{Element, Subscription, Task, time};

use crate::audio::AudioEngine;
use crate::cli::Cli;
use channel::Channel;

pub struct Loom {
    channel: Channel,
    audio_engine: AudioEngine,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    SourceSelected(String),
    SinkSelected(String),
    LevelsChanged(f32, f32),
}

impl Loom {
    pub fn new() -> Self {
        Self {
            channel: Channel::new(),
            audio_engine: AudioEngine::new(Cli::parse()),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                while let Some(event) = self.audio_engine.try_recv() {
                    self.channel.handle_event(event);
                }

                Task::none()
            }
            _ => self.channel.update(message),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content = row![self.channel.view()].spacing(20).padding(10);
        column![content].into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(50)).map(|_| Message::Tick)
    }
}
