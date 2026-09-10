mod cli;

use std::{error::Error, mem::MaybeUninit};

use crate::cli::Cli;
use clap::Parser;
use cpal::traits::{DeviceTrait, HostTrait};
use pipewire::{
    properties::properties,
    spa::{self, __builder_add__, pod::builder::Builder, utils::Direction},
    stream::StreamBox,
};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    dbg!(cli);

    let host = cpal::default_host();
    let devices = host.devices()?;

    let default_input_device = host
        .default_input_device()
        .ok_or("no default input device")?;

    let default_output_device = host
        .default_output_device()
        .ok_or("no default output device")?;

    for device in devices {
        let is_default_input = device == default_input_device;
        let is_default_output = device == default_output_device;
        dbg!(
            device.to_string(),
            device.description()?,
            is_default_input,
            is_default_output
        );
    }

    pipewire::init();

    let main_loop = pipewire::main_loop::MainLoopBox::new(None)?;
    let context = pipewire::context::ContextBox::new(main_loop.loop_(), None)?;
    let core = context.connect(None)?;
    let registry = core.get_registry()?;

    let properties = properties! {
        "media.class" => "Audio/Source",
        "node.name" => "loom_source",
        "node.description" => "loom_source virtual audio source",
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

    let stream = StreamBox::new(&core, "loom_source", properties)?;
    stream.connect(
        Direction::Output,
        None,
        pipewire::stream::StreamFlags::AUTOCONNECT,
        &mut [pod],
    )?;

    let listener = registry
        .add_listener_local()
        .global(|global| {
            println!("id={} type={}/{}", global.id, global.type_, global.version,);

            let Some(props) = global.props else {
                return;
            };

            match props.get("media.class") {
                Some("Stream/Output/Audio") => {
                    println!("audio source (stream): {:?}", props.get("application.name"));
                }
                Some("Audio/Source") => {
                    println!("audio source: {:?}", props.get("node.description"));
                }
                Some("Audio/Sink") => {
                    println!("audio sink: {:?}", props.get("node.description"));
                }
                _ => {}
            }
        })
        .register();

    main_loop.run();

    Ok(())
}
