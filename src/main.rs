//! penrose :: minimal configuration
//!
//! This file will give you a functional if incredibly minimal window manager that
//! has multiple workspaces and simple client / workspace movement.
use penrose::{
    builtin::{
        actions::{exit, modify_with, send_layout_message, spawn},
        layout::messages::{ExpandMain, IncMain, ShrinkMain},
    },
    core::{
        bindings::{parse_keybindings_with_xmodmap, KeyEventHandler},
        Config, WindowManager, ClientSet
    },
    extensions::hooks::add_ewmh_hooks,
    map,
    x11rb::RustConn,
    Result,
};
use std::collections::HashMap;
use tracing_subscriber::{self, prelude::*};

fn raw_key_bindings() -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
    let mut raw_bindings = map! {
        map_keys: str::to_string;

        "A-j" => modify_with(ClientSet::focus_down),
        "A-k" => modify_with(ClientSet::focus_up),
        "A-S-j" => modify_with(ClientSet::swap_down),
        "A-S-k" => modify_with(ClientSet::swap_up),
        "A-S-c" => modify_with(ClientSet::kill_focused),
        "A-Tab" => modify_with(ClientSet::toggle_tag),
        "A-bracketright" => modify_with(ClientSet::next_screen),
        "A-bracketleft" => modify_with(ClientSet::previous_screen),
        "A-grave" => modify_with(ClientSet::next_layout),
        "A-S-grave" => modify_with(ClientSet::previous_layout),
        "A-S-Up" => send_layout_message(|| IncMain(1)),
        "A-S-Down" => send_layout_message(|| IncMain(-1)),
        "A-S-Right" => send_layout_message(|| ExpandMain),
        "A-S-Left" => send_layout_message(|| ShrinkMain),
        "A-p" => spawn("rofi -show run"),
        "A-S-Return" => spawn("alacritty"),
        "A-S-q" => exit(),
    };

    for tag in &["1", "2", "3", "4", "5", "6", "7", "8", "9"] {
        raw_bindings.extend([
            (
                format!("A-{tag}"),
                modify_with(move |client_set| client_set.focus_tag(tag)),
            ),
            (
                format!("A-S-{tag}"),
                modify_with(move |client_set| client_set.move_focused_to_tag(tag)),
            ),
        ]);
    }

    raw_bindings
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();

    let conn = RustConn::new()?;
    let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings())?;
    let config = add_ewmh_hooks(Config::default());

    let wm = WindowManager::new(config, key_bindings, HashMap::new(), conn)?;

    wm.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindings_parse_correctly_with_xmodmap() {
        let res = parse_keybindings_with_xmodmap(raw_key_bindings());

        if let Err(e) = res {
            panic!("{e}");
        }
    }
}
