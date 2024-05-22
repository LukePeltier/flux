use std::{
    char,
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use zellij_tile::prelude::*;

fn is_directory(path_str: &String) -> bool {
    let path = Path::new(&path_str);
    path.exists() && path.is_dir()
}

#[derive(Debug)]
pub struct SessionShortcut {
    pub session_name: String,
    pub session_loc: PathBuf,
    pub session_keystroke: char,
}

#[derive(Default)]
struct State {
    shortcuts: Vec<SessionShortcut>,
}
register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
        for (key, value) in &configuration {
            if key.starts_with("keybind_") && is_directory(&value) {
                let session_name = value.replace(".", "_");
                self.shortcuts.push(SessionShortcut {
                    session_name,
                    session_loc: Path::new(value).to_owned(),
                    session_keystroke: key
                        .strip_prefix("keybind_")
                        .unwrap()
                        .chars()
                        .next()
                        .unwrap(),
                })
            }
        }
        subscribe(&[EventType::Key, EventType::SessionUpdate]);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Key(Key::Char(c)) => {
                if let Some(shortcut) = self
                    .shortcuts
                    .iter()
                    .find(|&shortcut| shortcut.session_keystroke == c)
                {
                    switch_session_with_layout(
                        Some(shortcut.session_name.as_str()),
                        LayoutInfo::File("default".to_string()),
                        Some(shortcut.session_loc.clone()),
                    )
                }
            }
            _ => (),
        };
        false
    }
}
