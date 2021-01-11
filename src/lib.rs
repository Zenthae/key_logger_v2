use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winit::event::VirtualKeyCode;

pub type RecordHolder = HashMap<Input, Vec<DateTime<Utc>>>;

/// Representation of an user input.
#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
#[serde(untagged)]
pub enum Input {
    Click(MouseClick),
    Key(VirtualKeyCode),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MouseClick {
    LeftClick,
    MiddleClick,
    RightClick,
}

// /// Representation of an user input.
// #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
// #[serde(into = "String")]
// #[serde(from = "String")]
// // #[serde(from = "&str")]
// pub enum Input {
//     Key(winit::event::VirtualKeyCode),
//     LeftClick,
//     MiddleClick,
//     RightClick,
// }

// impl Into<String> for Input {
//     fn into(self) -> String {
//         match self {
//             Input::LeftClick => "LeftClick".to_string(),
//             Input::MiddleClick => "MiddleClick".to_string(),
//             Input::RightClick => "RightClick".to_string(),
//             Input::Key(key) => format!("{:?}", key),
//         }
//     }
// }

// impl From<String> for Input {
//     fn from(s: String) -> Self {
//         if s.contains("LeftClick") {
//             return Input::LeftClick;
//         } else if s.contains("MiddleClick") {
//             return Input::MiddleClick;
//         } else if s.contains("RightClick") {
//             return Input::RightClick;
//         } else {
//             let key_code: VirtualKeyCode =
//                 serde_json::from_str(&format!("\"{}\"", s)).expect("Failed to parse keycode");
//             return Input::Key(key_code);
//         }
//     }
// }
