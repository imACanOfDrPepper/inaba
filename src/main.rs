mod activate_linux;
mod app_launcher;
mod bar;
mod lock;
mod notifs;
pub mod palette;
mod widgets;

use crate::activate_linux::activate_linux;
use crate::app_launcher::{app_launcher, toggle_launcher};
use crate::bar::bar;
use crate::lock::{lock, lock_screen};
use crate::notifs::{clear_notifs, notifs};
use crate::widgets::{media, system};

use amane::App;

fn main() {
    App::new()
        .window(bar)
        .window(activate_linux)
        .window(app_launcher)
        .window(notifs)
        .window(media)
        .window(system)
        .ipc("toggle-launcher", toggle_launcher)
        .ipc("lock-screen", lock)
        .ipc("clear-notifs", clear_notifs)
        .lock(lock_screen)
        .font("JetBrains Mono")
        .run();
}
