// Switch between "windows" and "console" to hide the console
#![windows_subsystem = "windows"]

use winit::{
    dpi::LogicalSize,
    error::OsError,
    event::{DeviceEvent, ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use chrono::{DateTime, Local, Utc};
use key_logger_v2::Input;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader};

type RecordHolder = HashMap<Input, Vec<DateTime<Utc>>>;

const DATA_FILE_NAME: &str = "data.json";

fn main() {
    std::panic::set_hook(Box::new(|panic_info| {
        if let Ok(mut error_log) =
            File::create(format!("crash-{}.txt", Local::now().format("%F_%H.%M.%S")))
        {
            error_log
                .write(format!("{}", panic_info).as_bytes())
                .expect("failed to fail ?");
        }
    }));

    let event_loop = EventLoop::new();
    let _window = window_builder(&event_loop).expect("Failed to create the window");

    let mut data: RecordHolder = HashMap::new();

    if let Ok(file) = File::open(DATA_FILE_NAME) {
        println!("Reading old content from file");
        data.extend(load_old_record(&file));
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        let input = match event {
            Event::DeviceEvent {
                event:
                    DeviceEvent::Button {
                        button,
                        state: ElementState::Released,
                    },
                ..
            } => match button {
                1 => Input::LeftClick,
                2 => Input::MiddleClick,
                3 => Input::RightClick,
                _ => return,
            },

            Event::DeviceEvent {
                event:
                    DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(key),
                        state: ElementState::Released,
                        ..
                    }),
                ..
            } => Input::Key(key),

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                let log_file = OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open(DATA_FILE_NAME)
                    .expect("Failed to open data file");

                if let Err(err) = serde_json::to_writer_pretty(&log_file, &data) {
                    eprintln!("Something went wrong while saving ! {:#?}", err);
                }

                *control_flow = ControlFlow::Exit;
                return;
            }
            _ => return,
        };

        data.entry(input).or_insert(Vec::new()).push(Utc::now());
    });
}

/// Build and return the program window with default settings
fn window_builder(event_loop: &EventLoop<()>) -> Result<Window, OsError> {
    let window = Window::new(&event_loop)?;
    window.set_title("Key Logger V2");
    window.set_inner_size(LogicalSize::new(300, 65));
    window.set_resizable(false);

    Ok(window)
}

/// Load old record from file `f` and return the content
fn load_old_record(f: &File) -> RecordHolder {
    let mut content = String::new();
    BufReader::new(f)
        .read_to_string(&mut content)
        .expect("Failed to read data file");

    if content.is_empty() {
        content = String::from("{}");
    }
    let old_data: RecordHolder = serde_json::from_str(&content).expect("Failed to parse old data");

    old_data
}
