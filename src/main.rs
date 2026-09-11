mod audio;
mod cli;

use std::{error::Error, rc::Rc, time::Duration};

use slint::{ModelRc, Timer, VecModel};

use crate::audio::AudioEngine;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let audio_engine = AudioEngine::new();

    let window = MainWindow::new()?;

    slint::set_xdg_app_id("loom")?;

    let sources = Rc::new(VecModel::from(vec!["none".into()]));
    let audio_state = window.global::<AudioState>();
    audio_state.set_sources(ModelRc::new(sources.clone()));

    let timer = Timer::default();

    timer.start(slint::TimerMode::Repeated, Duration::from_millis(50), {
        move || {
            println!("TICK");
            while let Some(event) = audio_engine.try_recv() {
                match event {
                    audio::AudioEvent::SourceAdded(source) => {
                        println!("source added: {}", source);
                        sources.push(source.nick.into());
                    }
                    // audio::AudioEvent::SourceRemoved(source_id) => {
                    //     println!("source removed: {}", source_id);
                    //     sources.push(source_id);
                    // }
                    audio::AudioEvent::SourceRemoved(audio_source_id) => {}
                    audio::AudioEvent::SinkAdded(audio_sink) => {}
                    audio::AudioEvent::SinkRemoved(audio_sink_id) => {}
                }
            }
        }
    });

    window.run()?;

    Ok(())
}
