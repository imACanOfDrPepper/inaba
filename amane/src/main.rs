mod activate_linux;
mod bar;

use crate::activate_linux::activate_linux;
use crate::bar::bar;

use amane::App;

fn main() {
    App::new()
        .window_per_monitor(bar)
        .window_per_monitor(activate_linux)
        .run();
}
