mod activate_linux;
mod app_launcher;
mod bar;
mod lock;
mod notifs;

use std::sync::LazyLock;

use crate::activate_linux::activate_linux;
use crate::app_launcher::{app_launcher, toggle_launcher};
use crate::bar::bar;
use crate::lock::{lock, lock_screen};
use crate::notifs::{clear_notifs, notifs};

use amane::App;

static SCALE: LazyLock<f32> = std::sync::LazyLock::new(|| std::env::var("AMANE_SCALE_FACTOR")
    .map(|value| value.parse::<f32>().expect("invalid AMANE_SCALE_FACTOR"))
    .unwrap_or(1.0));

fn main() {
    toggle_launcher(&["".to_string()]);

    App::new()
        .window(bar)
        .window(activate_linux)
        .window(app_launcher)
        .window(notifs)
        .ipc("toggle-launcher", toggle_launcher)
        .ipc("lock-screen", lock)
        .ipc("clear-notifs", clear_notifs)
        .lock(lock_screen)
        .scale_factor(*SCALE)
        .font("JetBrains Mono") 
        .run();
}
