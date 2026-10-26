mod activate_linux;
mod app_launcher;
mod bar;

use crate::activate_linux::activate_linux;
use crate::app_launcher::{app_launcher, toggle};
use crate::bar::bar;

use amane::App;

fn main() {
    toggle(&["".to_string()]);

    App::new()
        .window_per_monitor(bar)
        .window_per_monitor(activate_linux)
        .window(app_launcher)
        .ipc("toggle-launcher", toggle)
        .run();
}
