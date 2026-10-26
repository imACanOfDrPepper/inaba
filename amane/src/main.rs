mod activate_linux;
mod app_launcher;
mod bar;
mod notifs;

use crate::activate_linux::activate_linux;
use crate::app_launcher::{app_launcher, toggle};
use crate::bar::bar;
use crate::notifs::notifs;

use amane::App;

fn main() {
    toggle(&["".to_string()]);

    App::new()
        .window_per_monitor(bar)
        .window_per_monitor(activate_linux)
        .window(app_launcher)
        .window_per_monitor(notifs)
        .ipc("toggle-launcher", toggle)
        .run();
}
