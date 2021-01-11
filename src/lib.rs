use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winit::event::VirtualKeyCode;

pub type RecordHolder = HashMap<Input, Vec<DateTime<Utc>>>;

/// Representation of an user input.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(into = "String")]
#[serde(from = "&str")]
pub enum Input {
    Key(winit::event::VirtualKeyCode),
    LeftClick,
    MiddleClick,
    RightClick,
}

impl Into<String> for Input {
    fn into(self) -> String {
        match self {
            Input::LeftClick => "LeftClick".to_string(),
            Input::MiddleClick => "MiddleClick".to_string(),
            Input::RightClick => "RightClick".to_string(),
            Input::Key(key) => format!("{:?}", key),
        }
    }
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        match s {
            "LeftClick" => Input::LeftClick,
            "MiddleClick" => Input::MiddleClick,
            "RightClick" => Input::RightClick,
            // Ugly thing to do ugly behavior
            s => {
                let key_code: VirtualKeyCode =
                    serde_json::from_str(&format!("\"{}\"", s)).expect("Failed to parse keycode");
                Input::Key(key_code)
            }
        }
    }
}
