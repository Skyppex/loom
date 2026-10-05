use std::{
    cell::RefCell, collections::HashSet, error::Error, fmt::Display, mem::MaybeUninit, rc::Rc,
    sync::mpsc, thread,
};

use pipewire::{
    properties::properties,
    spa::{self, pod::builder::Builder, utils::Direction},
    stream::StreamBox,
};
use tracing::debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AudioSourceId(u32);

impl Display for AudioSourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AudioSource {
    pub id: AudioSourceId,
    pub nick: String,
    pub description: String,
}

impl Display for AudioSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} - {}", self.id, self.nick, self.description)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AudioSinkId(u32);

impl Display for AudioSinkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AudioSink {
    pub id: AudioSinkId,
    pub nick: String,
    pub description: String,
}

impl Display for AudioSink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.nick, self.description)
    }
}

pub enum AudioEvent {
    SourceAdded(AudioSource),
    SourceRemoved(AudioSourceId),
    SinkAdded(AudioSink),
    SinkRemoved(AudioSinkId),
}

pub struct AudioEngine {
    audio_events_rx: mpsc::Receiver<AudioEvent>,
}

impl AudioEngine {
    pub fn new(cli: Cli) -> Self {
        debug!("starting audio engine");
        let (audio_events_tx, audio_events_rx) = mpsc::channel::<AudioEvent>();

        thread::spawn(move || {
            let _ = audio_loop(audio_events_tx, cli);
        });

        Self { audio_events_rx }
    }

    pub fn try_recv(&self) -> Option<AudioEvent> {
        self.audio_events_rx.try_recv().ok()
    }
}

use crate::cli::Cli;

struct Playback {
    samples: Vec<f32>,
    position: usize,
}

fn audio_loop(
    audio_events_tx: mpsc::Sender<AudioEvent>,
    cli: Cli,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // let host = cpal::default_host();
    //
    // let devices = host.devices()?;
    //
    // let default_input_device = host
    //     .default_input_device()
    //     .ok_or("no default input device")?;
    //
    // let default_output_device = host
    //     .default_output_device()
    //     .ok_or("no default output device")?;
    //
    // for device in devices {
    //     let is_default_input = device == default_input_device;
    //     let is_default_output = device == default_output_device;
    //     dbg!(
    //         device.to_string(),
    //         device.description()?,
    //         is_default_input,
    //         is_default_output
    //     );
    // }

    pipewire::init();

    let main_loop = pipewire::main_loop::MainLoopBox::new(None)?;
    let context = pipewire::context::ContextBox::new(main_loop.loop_(), None)?;
    let core = context.connect(None)?;
    let registry = core.get_registry()?;

    let properties = properties! {
        "media.class" => "Audio/Source",
        "node.name" => cli.device_name.as_deref().unwrap_or("loom_source"),
        "node.description" => cli.device_name.as_deref().unwrap_or("loom_source").to_string() + " virtual audio source",
    };

    let mut buffer = vec![];
    let mut builder = Builder::new(&mut buffer);
    let mut frame = MaybeUninit::uninit();

    unsafe {
        builder.push_object(
            &mut frame,
            spa::utils::SpaTypes::ObjectParamFormat.as_raw(),
            spa::param::ParamType::EnumFormat.as_raw(),
        )?;
    }

    builder.add_prop(spa::param::format::FormatProperties::MediaType.as_raw(), 0)?;
    builder.add_id(spa::utils::Id(spa::param::format::MediaType::Audio.0))?;

    builder.add_prop(
        spa::param::format::FormatProperties::MediaSubtype.as_raw(),
        0,
    )?;
    builder.add_id(spa::utils::Id(spa::param::format::MediaSubtype::Raw.0))?;

    builder.add_prop(
        spa::param::format::FormatProperties::AudioFormat.as_raw(),
        0,
    )?;
    builder.add_id(spa::utils::Id(spa::param::audio::AudioFormat::F32LE.0))?;

    builder.add_prop(spa::param::format::FormatProperties::AudioRate.as_raw(), 0)?;
    builder.add_int(48000)?;

    builder.add_prop(
        spa::param::format::FormatProperties::AudioChannels.as_raw(),
        0,
    )?;
    builder.add_int(2)?;

    let pod_ptr = unsafe { builder.frame(frame.as_mut_ptr()) };

    unsafe {
        builder.pop(&mut frame.assume_init());
    }

    let pod = unsafe { spa::pod::Pod::from_raw(pod_ptr) };

    // let mut reader = hound::WavReader::open("/tmp/speakers.wav")?;

    // let samples: Vec<f32> = reader
    //     .samples::<i16>()
    //     .map(|sample| sample.unwrap() as f32 / i16::MAX as f32)
    //     .collect();

    // let playback = Playback {
    //     samples,
    //     position: 0,
    // };

    let stream = StreamBox::new(
        &core,
        cli.device_name.as_deref().unwrap_or("loom_source"),
        properties,
    )?;

    // let _stream_listener = stream
    //     .add_local_listener_with_user_data(playback)
    //     .process(|stream, playback| {
    //         let Some(mut buffer) = stream.dequeue_buffer() else {
    //             return;
    //         };
    //
    //         let datas = buffer.datas_mut();
    //         let Some(data) = datas.first_mut() else {
    //             return;
    //         };
    //
    //         let count = {
    //             let Some(slice) = data.data() else {
    //                 return;
    //             };
    //
    //             let output = unsafe {
    //                 std::slice::from_raw_parts_mut(
    //                     slice.as_mut_ptr() as *mut f32,
    //                     slice.len() / std::mem::size_of::<f32>(),
    //                 )
    //             };
    //
    //             let remaining = playback.samples.len() - playback.position;
    //             let count = remaining.min(output.len());
    //
    //             output[..count].copy_from_slice(
    //                 &playback.samples[playback.position..playback.position + count],
    //             );
    //
    //             output[count..].fill(0.0);
    //             playback.position += count;
    //             count
    //         };
    //
    //         let chunk = data.chunk_mut();
    //         *chunk.offset_mut() = 0;
    //         *chunk.stride_mut() = (2 * std::mem::size_of::<f32>()) as i32;
    //         *chunk.size_mut() = (count * std::mem::size_of::<f32>()) as u32;
    //     })
    //     .register()?;

    stream.connect(
        Direction::Output,
        None,
        pipewire::stream::StreamFlags::AUTOCONNECT,
        &mut [pod],
    )?;

    let added_ids = Rc::new(RefCell::new(HashSet::new()));

    let _listener = registry
        .add_listener_local()
        .global({
            let audio_events_tx = audio_events_tx.clone();
            let added_ids = added_ids.clone();

            move |global| {
                let Some(props) = global.props else {
                    return;
                };

                match props.get("media.class") {
                    Some("Stream/Output/Audio") => {
                        let Some(name) = props.get("application.name") else {
                            return;
                        };

                        let source = AudioSource {
                            id: AudioSourceId(global.id),
                            nick: name.to_owned(),
                            description: name.to_owned(),
                        };

                        tracing::info!("audio: source added: {}", &source);

                        audio_events_tx
                            .send(AudioEvent::SourceAdded(source))
                            .expect("failed to send event over a channel");

                        added_ids.borrow_mut().insert(global.id);
                    }
                    Some("Audio/Source") => {
                        let Some(nick) = props.get("node.nick") else {
                            return;
                        };

                        let description = props.get("node.description");

                        let source = AudioSource {
                            id: AudioSourceId(global.id),
                            nick: nick.to_owned(),
                            description: description
                                .map(|v| v.to_owned())
                                .unwrap_or_else(|| nick.to_owned()),
                        };

                        tracing::info!("audio: source added: {}", &source);

                        audio_events_tx
                            .send(AudioEvent::SourceAdded(source))
                            .expect("failed to send event over a channel");

                        added_ids.borrow_mut().insert(global.id);
                    }
                    Some("Audio/Sink") => {
                        println!("audio sink: {:?}", props.get("node.description"));

                        let Some(nick) = props.get("node.nick") else {
                            return;
                        };

                        let description = props.get("node.description");

                        let sink = AudioSink {
                            id: AudioSinkId(global.id),
                            nick: nick.to_owned(),
                            description: description
                                .map(|v| v.to_owned())
                                .unwrap_or_else(|| nick.to_owned()),
                        };

                        tracing::info!("audio: sink added: {}", &sink);

                        audio_events_tx
                            .send(AudioEvent::SinkAdded(sink))
                            .expect("failed to send event over a channel");

                        added_ids.borrow_mut().insert(global.id);
                    }
                    _ => {}
                }
            }
        })
        .global_remove({
            let audio_events_tx = audio_events_tx.clone();
            let added_ids = added_ids.clone();

            move |id| {
                if !added_ids.borrow().contains(&id) {
                    return;
                }

                audio_events_tx
                    .send(AudioEvent::SourceRemoved(AudioSourceId(id)))
                    .expect("failed to send event over a channel");
            }
        })
        .register();

    main_loop.run();

    Ok(())
}
