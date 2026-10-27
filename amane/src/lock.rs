use amane::{Center, Color, Column, Image, LayerWindow, Lock, Monitor, Rectangle, Service, Size::Parent, Text, TextInput, children};

pub fn lock_screen(_: &Monitor) -> LayerWindow {
    let lock = Lock::read();

    let status = if lock.checking() {
        "Checking..."
    } else if lock.failed() {
        "Wrong password :("
    } else {
        "Locked"
    };

    let text_color = if lock.failed() {
        Color::rgb(255, 153, 153)
    } else {
        Color::rgb(221, 221, 221)
    };

    LayerWindow::new().width(1.).height(1.).child(
        Rectangle::new()
            .width(Parent)
            .height(Parent)
            .fill(Image::cover("/home/prohaker/Pictures/Wallpapers/xenia.png").thumbnail(64, 36).blurred(2))
            .align_child(Center, Center)
            .child(
                Column::new(children![
                    Text::new(status).size(24.).color(text_color),
                    Rectangle::new()
                        .width(300.)
                        .height(40.)
                        .fill("#b2b2b2")
                        .radius(12.)
                        .padding(8.)
                        .child(
                            TextInput::new("password")
                                .placeholder("Password")
                                .password()
                                .focused()
                                .color(Color::rgb(51, 51, 51))
                                .on_submit(|password| Lock::unlock(&password)),
                        ),
                ])
                .gap(12.),
            ),
    )
}

pub fn lock(_: &[String]) -> String {
    Lock::start();

    String::from("locked")
}
