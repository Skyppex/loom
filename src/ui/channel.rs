use std::collections::HashMap;

use iced::widget::{column, container, pick_list, row, text, vertical_slider};
use iced::{Element, Length, Task};

use crate::audio::AudioEvent;

use super::Message;
use super::stereo_visual;

pub struct Channel {
    sources: Vec<String>,
    sinks: Vec<String>,
    source_map: HashMap<String, String>,
    sink_map: HashMap<String, String>,
    selected_source: Option<String>,
    selected_sink: Option<String>,
    volume: f32,
}

impl Channel {
    pub fn new() -> Self {
        Self {
            sources: vec!["none".into()],
            sinks: vec!["none".into()],
            source_map: HashMap::new(),
            sink_map: HashMap::new(),
            selected_source: None,
            selected_sink: None,
            volume: 1.0,
        }
    }

    /// Handle audio events coming from the PipeWire thread.
    /// Called by the parent `Loom` on each `Tick`.
    pub fn handle_event(&mut self, event: AudioEvent) {
        tracing::info!("ui: handling audio event");

        match event {
            AudioEvent::SourceAdded(source) => {
                println!("source added: {}", source);
                let id_str = source.id.to_string();
                let nick = source.nick.clone();
                self.source_map.insert(id_str, nick.clone());
                self.sources.push(nick);
            }
            AudioEvent::SourceRemoved(id) => {
                let id_str = id.to_string();
                if let Some(nick) = self.source_map.remove(&id_str) {
                    self.sources.retain(|n| n != &nick);
                    if self.selected_source.as_ref() == Some(&nick) {
                        self.selected_source = None;
                    }
                    println!("source removed: {}", nick);
                } else if let Some(nick) = self.sink_map.remove(&id_str) {
                    // Workaround: audio module sends SourceRemoved for sinks as well
                    self.sinks.retain(|n| n != &nick);
                    if self.selected_sink.as_ref() == Some(&nick) {
                        self.selected_sink = None;
                    }
                    println!("sink removed (via SourceRemoved): {}", nick);
                } else {
                    eprintln!("source {} not found", id);
                }
            }
            AudioEvent::SinkAdded(sink) => {
                println!("sink added: {}", sink);
                let id_str = sink.id.to_string();
                let nick = sink.nick.clone();
                self.sink_map.insert(id_str, nick.clone());
                self.sinks.push(nick);
            }
            AudioEvent::SinkRemoved(id) => {
                let id_str = id.to_string();
                if let Some(nick) = self.sink_map.remove(&id_str) {
                    self.sinks.retain(|n| n != &nick);
                    if self.selected_sink.as_ref() == Some(&nick) {
                        self.selected_sink = None;
                    }
                    println!("sink removed: {}", nick);
                } else {
                    eprintln!("sink {} not found", id);
                }
            }
        }
    }

    /// Handle UI messages. Returns a Task for the iced runtime.
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SourceSelected(name) => {
                eprintln!("1={}", name);
                self.selected_source = Some(name);
            }
            Message::SinkSelected(name) => {
                self.selected_sink = Some(name);
            }
            Message::VolumeChanged(v) => {
                self.volume = v;
            }
            Message::Tick => {
                // Tick is handled by the parent; nothing to do here
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        // Fixed channel width so centering is inside the channel, not the window
        let channel_width = Length::Fixed(220.0);

        let source_pick = pick_list(
            self.sources.clone(),
            self.selected_source.clone(),
            Message::SourceSelected,
        )
        .placeholder("Select source")
        .width(channel_width);

        let sink_pick = pick_list(
            self.sinks.clone(),
            self.selected_sink.clone(),
            Message::SinkSelected,
        )
        .placeholder("Select sink")
        .width(channel_width);

        let slider = vertical_slider(0.0..=1.0, self.volume, Message::VolumeChanged)
            .height(200.0)
            .step(0.01);

        let meter = stereo_visual::view(self.volume, self.volume);

        let controls = container(
            row![meter, slider]
                .spacing(10)
                .align_y(iced::Alignment::Center),
        )
        .center_x(channel_width);

        column![text("Channel 1").size(16), source_pick, controls, sink_pick,]
            .spacing(10)
            .padding(20)
            .width(Length::Shrink)
            .into()
    }
}
