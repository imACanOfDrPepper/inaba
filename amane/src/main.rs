mod activate_linux;
mod app_launcher;
mod bar;
mod notifs;
mod lock;

use crate::activate_linux::activate_linux;
use crate::app_launcher::{app_launcher, toggle_launcher};
use crate::bar::bar;
use crate::notifs::notifs;
use crate::lock::{lock, lock_screen};

use amane::App;

fn main() {
    toggle_launcher(&["".to_string()]);

    App::new()
        .window_per_monitor(bar)
        .window_per_monitor(activate_linux)
        .window(app_launcher)
        .window_per_monitor(notifs)
        .ipc("toggle-launcher", toggle_launcher)
        .ipc("lock-screen", lock)
        .lock(lock_screen)
        .run();
}
