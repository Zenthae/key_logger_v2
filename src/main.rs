// Switch between "windows" and "console" to hide the console
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{BufWriter, Write},
    path::PathBuf,
};

use chrono::Utc;
use key_logger_v2::{Input, MouseClick};
use winit::{
    dpi::LogicalSize,
    event::{DeviceEvent, ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

const APP_NAME: &str = "Key Logger";
const APP_VERSION: &str = "2";
const APP_ID: &str = "key_logger_v2";
const VENDOR: &str = "Zenthae";
const LOG_FILE_NAME: &str = "records.log";

fn main() {
    // Switch to `with_capacity` if the default capacity doesn't fit
    // 1024b =~ 25event
    let mut bw = BufWriter::with_capacity(1 * 1024, file());

    let event_loop = EventLoop::with_user_event();
    let _window = window_builder(&event_loop);

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
                1 => Input::Click(MouseClick::LeftClick),
                2 => Input::Click(MouseClick::MiddleClick),
                3 => Input::Click(MouseClick::RightClick),
                _ => return,
            },

            Event::DeviceEvent {
                event:
                    DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(key_code),
                        state: ElementState::Released,
                        ..
                    }),
                ..
            } => Input::Key(key_code),

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            }
            | Event::WindowEvent {
                event: WindowEvent::EndSession,
                ..
            } => {
                // Flush the buffer writer when the application is closing to save the unsaved data.
                bw.flush().expect("Failed to flush, sad but true.");
                return *control_flow = ControlFlow::Exit;
            }
            _ => return,
        };

        println!("{}", bw.buffer().len());

        serde_json::to_writer(&mut bw, &(input, Utc::now())).expect("Failed to write input");
    });
}

/// Retrieve the app data path.
fn path() -> PathBuf {
    let mut path = if let Some(dirs) = directories_next::ProjectDirs::from("", VENDOR, APP_ID) {
        dirs.data_dir().into()
    } else {
        env::current_dir().unwrap_or(PathBuf::new())
    };

    path.push(LOG_FILE_NAME);

    path
}

fn file() -> File {
    let path = path();

    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).expect("Failed to create data directory");
    };

    OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)
        .unwrap()
}

fn window_builder(event_loop: &EventLoop<()>) -> Window {
    let window = Window::new(&event_loop).expect("Failed to generate the window");

    window.set_title(format!("{} v{}", APP_NAME, APP_VERSION).as_str());
    window.set_inner_size(LogicalSize::new(300, 65));
    window.set_resizable(false);

    window
}
