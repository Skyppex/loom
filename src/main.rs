mod audio;
mod cli;

use std::{collections::HashMap, error::Error, rc::Rc, time::Duration};

use slint::{Model, ModelRc, Timer, VecModel};

use crate::audio::AudioEngine;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let audio_engine = AudioEngine::new();

    let window = MainWindow::new()?;

    slint::set_xdg_app_id("loom")?;

    let source_names = Rc::new(VecModel::from(vec!["none".into()]));
    // let source_ids = Rc::new(VecModel::from(vec![]));
    let sink_names = Rc::new(VecModel::from(vec!["none".into()]));
    // let sink_ids = Rc::new(VecModel::from(vec![]));

    let audio_state = window.global::<AudioState>();
    audio_state.set_source_names(ModelRc::new(source_names.clone()));
    audio_state.set_sink_names(ModelRc::new(sink_names.clone()));

    let mut audio_sources = HashMap::new();
    let mut audio_sinks = HashMap::new();

    let timer = Timer::default();

    timer.start(slint::TimerMode::Repeated, Duration::from_millis(50), {
        move || {
            while let Some(event) = audio_engine.try_recv() {
                match event {
                    audio::AudioEvent::SourceAdded(source) => {
                        println!("source added: {}", source);
                        source_names.push(source.nick.clone().into());
                        // source_ids.push(source.id);
                        audio_sources.insert(source.id, (source, source_names.row_count() - 1));
                    }
                    audio::AudioEvent::SourceRemoved(audio_source_id) => {
                        dbg!(&audio_source_id, &audio_sources);

                        let Some((source, index)) = audio_sources.remove(&audio_source_id) else {
                            eprintln!("source {} not found", audio_source_id);
                            continue;
                        };

                        source_names.remove(index);
                        // source_ids.remove(index);
                        println!("source removed: {}", source);
                    }
                    audio::AudioEvent::SinkAdded(sink) => {
                        println!("sink added: {}", sink);
                        sink_names.push(sink.nick.clone().into());
                        // sink_ids.push(sink.id);
                        audio_sinks.insert(sink.id, (sink, sink_names.row_count() - 1));
                    }
                    audio::AudioEvent::SinkRemoved(audio_sink_id) => {
                        let Some((sink, index)) = audio_sinks.remove(&audio_sink_id) else {
                            eprintln!("sink {} not found", audio_sink_id);
                            continue;
                        };

                        sink_names.remove(index);
                        // sink_ids.remove(index);
                        println!("sink removed: {}", sink);
                    }
                }
            }
        }
    });

    window.run()?;

    Ok(())
}
